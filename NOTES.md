The git tag just helps us find the state of the repository at the time of the
last release. It has no relation to the individual versions of the packages
within it.

Having said that, packages can pin their versions to the [workspace] version,
meaning that for that package, we actually have to update the workspace version.
If multiple packages inherit it, we have to incremement it by the greatest
change (i.e if package_d has fix, and package_c has new feature, we bump the
workspace feature value.).

Based on what I've seen so far, `cargo` actually makes the view between single
packages and workspaces completely transparent. 

The problem with workspaces is that the initial worktree we create isn't
packaged the same way that that our latest release is. Packages within
workspaces cannot be meaninfully compared in isolation because they might pull
dependencies / other metadata from the workspace root. When we run `cargo
publish`, we hardcode any information from the workspace `Cargo.toml` into the
packages `Cargo.toml`. 

In this sense, we actually don't care about the workspace at all unless we need
to update the version because a package has pinned its version to the workspace
version.

One issue I think we have is that when determining what commits changed what
packages, we need to pretend we are actually in the workspace for the paths to
exist / make sense. 


Really all PR comes down to is getting a copy of all of packages at 2 different
points in time and doing stuff with them. Here's what we use them for:

Latest Released -> getting version numbers for that release


The unreleased_worktree is used to compare to the latest released worktree,
collecting commits until we reach the worktree at the time of the latest
release. We then determine which commits applied to which packages. 


Here is the simplified list of things we need to do: 

1. Get copies of all of the unreleased packages 
2. Get copies of the latest release for all packages
3. Inspect commits to determine which commits affected which packages 
4. Use that data + original versions to determine what the new versions should
   be 

For each package, the latest released versions may either come from git, or from
a registry of some sort depending on the CLI args passed and the config.

Finding the latest release for registry packages is simple, because you can just
ask for the latest. 

Finding the latest release through git is a little bit more challenging. We have
to determine the state of the repository at the time of the last release. This
can be done through tags. 

Filtering through git commits to determine which packages changed is done by
inspecting the paths of the files changed. Obviously if something is modified in
`/{package_name}` , we know it affected that package. But a more subtle
difference is that a change in the workspace `Cargo.toml` can actually be a
change in the package as well (i.e bumping a dep). This may not matter because
we are comparing 

I think the comparison function can also be made a bit simpler. All we need to
do is check the sha1 of the the current package and see if it matches the sha1
of the latest released package. 

When collecting commits, we can just determine if the sha1 of the current
package is the same as the previous one, if not, clearly something changed, and
so we add it to our commit list.

So our job pretty much becomes:

1. Read CLI args 
2. Read config file 
3. Combine those into some struct 
4. If any of the packages have `git_only` set to true, we get the repo at the
   state of the last release 
5. Get copies of all of the packages at the time of their last release
6. Starting at the current commit, run cargo package, for each of packages,
   first check if the sha is the same as the latest release package, if yes,
    then we are done with that package. If no, check if the sha1 is the same as
    last time around. If not, add the commit to the commit list for this
    package. Keep going until all packages are done.  
7. At this point, you have a commit list for each package that you can use to
   pick the next version


We should also try to run checks as quickly as possible. Not allowed to use
dirty? Check early.

If we want to allow empty commits, we have to keep checkings shas until they
DONT match.

For git only releases, if there is no match on the regex, then we have to find
the commit where the package is introduced, and use that instead. We still take
the version from that cargo.toml as with everything else.

Once we have all of the releases for packages, the process doesn't actually care
about the type of package (i.e git / registry). We just step commit by commit
and check the sha against all of our packages.


If no tags match regex, we actually have to scan all commits until we find that
it exists. We can use binary search!

use dirwalk to list files in a consistent order, check ripgrep

for packages with no release tag, then we create one worktree per and do a
binary search / git bisect until we find the commit when the package was first
introduced?

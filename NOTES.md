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

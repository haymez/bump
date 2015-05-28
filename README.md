#Bump
Simple Rust utility to handle bumping VERSION files.

##Installation
1. Install rust and cargo.
2. clone repo
3. run cargo build --release
4. copy target/release/bump (.exe if on windows) to directory of choice and add to path. For linux the directory would be /bin or ~/bin depending on how your system is set up.

###Current Features
* bumps major, minor, and patch versions depending on command line argument passed in

####Example
```shell
# Bump major version and zeroes out minor and patch
bump major

# Bump minor version and zeroes out patch
bump minor

# Bump patch version
bump patch
```
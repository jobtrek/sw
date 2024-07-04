# Changelog

## [0.1.5](https://github.com/jobtrek/sw/compare/v0.1.4...v0.1.5) (2024-07-03)


### Features

* add java language ([#24](https://github.com/jobtrek/sw/issues/24)) ([0266bd9](https://github.com/jobtrek/sw/commit/0266bd92947cc3b885a7fe3d57bfa29f10ebf02d))
* upgrade path selection ([#22](https://github.com/jobtrek/sw/issues/22)) ([e421db0](https://github.com/jobtrek/sw/commit/e421db002b35470ad7f795c796675e4f94fc942b))


### Refactors

* use an enum to recognize extensions instead of testing it in main() ([#21](https://github.com/jobtrek/sw/issues/21)) ([8ad79eb](https://github.com/jobtrek/sw/commit/8ad79eb68b0245879c5ffc7176855883f8d418a8))

## [0.1.4](https://github.com/jobtrek/sw/compare/v0.1.3...v0.1.4) (2024-07-02)


### Features

* add files with the expectated result of the tests ([#20](https://github.com/jobtrek/sw/issues/20)) ([63b2457](https://github.com/jobtrek/sw/commit/63b24575cea23b3f6d9718368867ee050096534c))
* add test expectation files ([63b2457](https://github.com/jobtrek/sw/commit/63b24575cea23b3f6d9718368867ee050096534c))
* add typescript ([#17](https://github.com/jobtrek/sw/issues/17)) ([fca07e0](https://github.com/jobtrek/sw/commit/fca07e0afa006bbed1d9f68b093cf880b9bb84f3))


### Miscellaneous Chores

* add license ([#15](https://github.com/jobtrek/sw/issues/15)) ([379f21e](https://github.com/jobtrek/sw/commit/379f21eb44db847eda9aeec7abbef1ba976de936))
* **deps:** bump clap from 4.5.7 to 4.5.8 ([#18](https://github.com/jobtrek/sw/issues/18)) ([6c344cb](https://github.com/jobtrek/sw/commit/6c344cb93846815f52e3d3b0d57e4ed5f1fc3703))
* **deps:** bump serde_json from 1.0.117 to 1.0.119 ([#19](https://github.com/jobtrek/sw/issues/19)) ([7ba4e86](https://github.com/jobtrek/sw/commit/7ba4e8670e341b2d53d8cfd8e37de53afc019539))


### Refactors

* move test and expected result in same folder ([63b2457](https://github.com/jobtrek/sw/commit/63b24575cea23b3f6d9718368867ee050096534c))

## [0.1.3](https://github.com/jobtrek/sw/compare/v0.1.2...v0.1.3) (2024-06-24)


### Features

* add command line arguments ([6611550](https://github.com/jobtrek/sw/commit/66115502b4d35dea8caf722a6f98084ecc81e84f))
* allow user to chose the file type ([6611550](https://github.com/jobtrek/sw/commit/66115502b4d35dea8caf722a6f98084ecc81e84f))
* allow user to chose to disable feedback ([6611550](https://github.com/jobtrek/sw/commit/66115502b4d35dea8caf722a6f98084ecc81e84f))

## [0.1.2](https://github.com/jobtrek/sw/compare/v0.1.1...v0.1.2) (2024-06-21)


### Features

* add support for the Javascript language ([#12](https://github.com/jobtrek/sw/issues/12)) ([9559804](https://github.com/jobtrek/sw/commit/955980448b8b22d9d49969f05dd03d38311529be))
* add toto!() macro to each functions ([9fbb77d](https://github.com/jobtrek/sw/commit/9fbb77dc754b058cda3d7ab052ca9fc3d3076fcd))
* create a developement version of the dockerfile ([9fbb77d](https://github.com/jobtrek/sw/commit/9fbb77dc754b058cda3d7ab052ca9fc3d3076fcd))
* get the lines number of the lines to remove ([9fbb77d](https://github.com/jobtrek/sw/commit/9fbb77dc754b058cda3d7ab052ca9fc3d3076fcd))
* indent replacing text according to the comment's indentation ([9fbb77d](https://github.com/jobtrek/sw/commit/9fbb77dc754b058cda3d7ab052ca9fc3d3076fcd))
* remove code after comments in rust ([#8](https://github.com/jobtrek/sw/issues/8)) ([9fbb77d](https://github.com/jobtrek/sw/commit/9fbb77dc754b058cda3d7ab052ca9fc3d3076fcd))
* remove the wanted code from the file ([9fbb77d](https://github.com/jobtrek/sw/commit/9fbb77dc754b058cda3d7ab052ca9fc3d3076fcd))
* update dockerfile ([9fbb77d](https://github.com/jobtrek/sw/commit/9fbb77dc754b058cda3d7ab052ca9fc3d3076fcd))


### Bug Fixes

* select definition scope with the other scopes ([9fbb77d](https://github.com/jobtrek/sw/commit/9fbb77dc754b058cda3d7ab052ca9fc3d3076fcd))


### Tests

* add the most relevent test done to the project files ([9fbb77d](https://github.com/jobtrek/sw/commit/9fbb77dc754b058cda3d7ab052ca9fc3d3076fcd))


### Performance Improvements

* remove variable to querry data directly when usefull ([9fbb77d](https://github.com/jobtrek/sw/commit/9fbb77dc754b058cda3d7ab052ca9fc3d3076fcd))


### Miscellaneous Chores

* setup codeowners ([#5](https://github.com/jobtrek/sw/issues/5)) ([3df853c](https://github.com/jobtrek/sw/commit/3df853c37d70662dda2df5567f1270d386b0e8c3))


### Documentation

* fix rust extension file ([#7](https://github.com/jobtrek/sw/issues/7)) ([838e182](https://github.com/jobtrek/sw/commit/838e182fde84e47679c8d276ca7956a0b24bb018))


### Refactors

* make all structure field snake_case ([9fbb77d](https://github.com/jobtrek/sw/commit/9fbb77dc754b058cda3d7ab052ca9fc3d3076fcd))


### Continuous Integration

* add concurrency group to avoid concurrent running ([#11](https://github.com/jobtrek/sw/issues/11)) ([21ff0d3](https://github.com/jobtrek/sw/commit/21ff0d3d864809fe3822a10c1e85c64dc70bacdc))
* setup dependabot ([#9](https://github.com/jobtrek/sw/issues/9)) ([225f515](https://github.com/jobtrek/sw/commit/225f5150b393334a25e15ba1e2d6d7fad3ea8524))

## [0.1.1](https://github.com/jobtrek/sw/compare/v0.1.0...v0.1.1) (2024-06-18)


### Miscellaneous Chores

* .idea folder in gitignore ([b2fc4dc](https://github.com/jobtrek/sw/commit/b2fc4dc62df7f0701b2d551b2d8969905cb5c873))
* cargo initialisation ([6c152b5](https://github.com/jobtrek/sw/commit/6c152b5c078f87f521fd83e076ca537b9b88ecce))


### Documentation

* initial readme ([8e9da77](https://github.com/jobtrek/sw/commit/8e9da77c1fd3296ef2d19789125c017679cf9995))
* sw defaults description ([0b75e6b](https://github.com/jobtrek/sw/commit/0b75e6b6e59ca1341036463a988ca4e9500474b8))


### Continuous Integration

* add clippy verification tu action ([b2fc4dc](https://github.com/jobtrek/sw/commit/b2fc4dc62df7f0701b2d551b2d8969905cb5c873))
* add release please ([#3](https://github.com/jobtrek/sw/issues/3)) ([a4222ff](https://github.com/jobtrek/sw/commit/a4222fff6017e55c5056f68aeb72b3b7827c828e))
* add release please configuration files ([a4222ff](https://github.com/jobtrek/sw/commit/a4222fff6017e55c5056f68aeb72b3b7827c828e))
* add release please releaser action ([a4222ff](https://github.com/jobtrek/sw/commit/a4222fff6017e55c5056f68aeb72b3b7827c828e))
* setup ci action ([#2](https://github.com/jobtrek/sw/issues/2)) ([b2fc4dc](https://github.com/jobtrek/sw/commit/b2fc4dc62df7f0701b2d551b2d8969905cb5c873))
* setup formatter and test check workflow ([b2fc4dc](https://github.com/jobtrek/sw/commit/b2fc4dc62df7f0701b2d551b2d8969905cb5c873))

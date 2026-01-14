# Changelog

## [0.5.4](https://github.com/jobtrek/sw/compare/v0.5.3...v0.5.4) (2026-01-14)


### Miscellaneous Chores

* **deps:** bump serde_json from 1.0.148 to 1.0.149 ([#169](https://github.com/jobtrek/sw/issues/169)) ([e146b88](https://github.com/jobtrek/sw/commit/e146b88185f17764c0b913886a9d2e884cdefef1))

## [0.5.3](https://github.com/jobtrek/sw/compare/v0.5.2...v0.5.3) (2026-01-06)


### Miscellaneous Chores

* **deps:** bump clap from 4.5.53 to 4.5.54 ([#168](https://github.com/jobtrek/sw/issues/168)) ([6cca9b1](https://github.com/jobtrek/sw/commit/6cca9b1c0ed650739a7240436c8e8fb6ddcd1255))
* **deps:** bump serde_json from 1.0.145 to 1.0.148 ([#166](https://github.com/jobtrek/sw/issues/166)) ([dc146e1](https://github.com/jobtrek/sw/commit/dc146e1b0be252fc533f98fce7a4c66ca7d9c292))

## [0.5.2](https://github.com/jobtrek/sw/compare/v0.5.1...v0.5.2) (2025-12-18)


### Miscellaneous Chores

* **deps:** bump alpine from 3.22 to 3.23 in the docker-minor group ([#163](https://github.com/jobtrek/sw/issues/163)) ([5a72937](https://github.com/jobtrek/sw/commit/5a72937e4633caa42d57ab22de78c666f60176c8))
* **deps:** bump rust in the docker-minor group ([#165](https://github.com/jobtrek/sw/issues/165)) ([505100b](https://github.com/jobtrek/sw/commit/505100b734bb437ff2d30a6565919d3e8576a3ab))

## [0.5.1](https://github.com/jobtrek/sw/compare/v0.5.0...v0.5.1) (2025-11-29)


### Miscellaneous Chores

* **deps:** bump actions/checkout from 5 to 6 ([#161](https://github.com/jobtrek/sw/issues/161)) ([e070bf9](https://github.com/jobtrek/sw/commit/e070bf936f488993a617f2780d590be37020684b))
* **deps:** bump clap from 4.5.51 to 4.5.53 ([#160](https://github.com/jobtrek/sw/issues/160)) ([0fb26c5](https://github.com/jobtrek/sw/commit/0fb26c5a4c43a2f3e480b1fc9ea5fe24ed346a59))

## [0.5.0](https://github.com/jobtrek/sw/compare/v0.4.16...v0.5.0) (2025-11-06)


### Features

* change message for list of failing tests ([0e1ba00](https://github.com/jobtrek/sw/commit/0e1ba005d557245e52727bebffe7b73125df4189))
* make scripts work when being sourced ([e9ceb47](https://github.com/jobtrek/sw/commit/e9ceb47aebe59bb73f9f95691ec8248604fc7c9d))


### Bug Fixes

* determine fd cmd based on existance of fdfind command ([5c6e7ce](https://github.com/jobtrek/sw/commit/5c6e7ce2889a8b2d60bea17ccf66334c24611bd8))
* shebang ([5beda07](https://github.com/jobtrek/sw/commit/5beda0743a3e04378706464a80bea0281543b56d))


### Tests

* add e2e tests ([55b738b](https://github.com/jobtrek/sw/commit/55b738bfd06dcb946e39fe733a04480b28a25966))
* add e2e tests ([2c1f1bf](https://github.com/jobtrek/sw/commit/2c1f1bf356c7d69c43c546a4c29e91d72827f09a))
* check if fd is installed ([12a8ac7](https://github.com/jobtrek/sw/commit/12a8ac76d1722325ee32709787ae218b3d26a0dd))
* crash if any command fails ([2386d97](https://github.com/jobtrek/sw/commit/2386d97f105df0e91769ee4e41cdd83a71f7eb6c))
* crash if any command fails ([73e48be](https://github.com/jobtrek/sw/commit/73e48be14f93147ad5ee6e90969833f84cd9e84d))
* differentiate diff error and diff between folders ([b249855](https://github.com/jobtrek/sw/commit/b24985575088e66453e79029ebe55cd8ecf1df67))
* don't rebuild program for each e2e test ([c78c7c2](https://github.com/jobtrek/sw/commit/c78c7c21915c72e6dbd32146fc9d7c0d304d4af8))
* run cleanup even if program crashes ([224e1ec](https://github.com/jobtrek/sw/commit/224e1ec3f31af23bdcab141ce838c12faabf8587))


### Performance Improvements

* don't store fd's result in variable ([a333a27](https://github.com/jobtrek/sw/commit/a333a2725a761e1f065f5e01c6d0f3ee44bb044f))


### Miscellaneous Chores

* **deps:** bump clap from 4.5.48 to 4.5.50 ([#156](https://github.com/jobtrek/sw/issues/156)) ([0a38b0f](https://github.com/jobtrek/sw/commit/0a38b0fee031af89a7cc0d3137ca3cb88a2e5381))
* **deps:** bump clap from 4.5.50 to 4.5.51 ([#158](https://github.com/jobtrek/sw/issues/158)) ([b8ebcd9](https://github.com/jobtrek/sw/commit/b8ebcd92887edfc3ab8fb8f37624ab15fc9bd56d))
* **deps:** bump rust in the docker-minor group ([#159](https://github.com/jobtrek/sw/issues/159)) ([e247d48](https://github.com/jobtrek/sw/commit/e247d4885fc338cfbd495f7a33826e4e1e8d47aa))
* recursive diff by default ([a7358ec](https://github.com/jobtrek/sw/commit/a7358ec534945d91c01fb1a0d6bb0cdda01c1d6d))
* remove useless cleanup call ([1d55061](https://github.com/jobtrek/sw/commit/1d55061379d5cc274537d4d486c2c1a46c9ded1a))
* use \0 instead of \n between fd and read ([6026f0f](https://github.com/jobtrek/sw/commit/6026f0f70fbb9b198cfdb115a73fbc0e6f3c9907))


### Refactors

* call diff tool in if statement ([0346ca3](https://github.com/jobtrek/sw/commit/0346ca36773cb7986fb7b9c7fb498ea91a54240b))
* don't simlink fdfind = fd ([df8d6dd](https://github.com/jobtrek/sw/commit/df8d6dd29cf2f31894b67714b7f3b2fde8acad1a))


### Continuous Integration

* e2e tests in pipeline ([11442be](https://github.com/jobtrek/sw/commit/11442be6818245217ff969472e710729423f8f0f))
* forces e2e to run with bash ([2201c43](https://github.com/jobtrek/sw/commit/2201c43610b932c0f6195a085e9dbbce62abb3e4))

## [0.4.16](https://github.com/jobtrek/sw/compare/v0.4.15...v0.4.16) (2025-09-29)


### Miscellaneous Chores

* **deps:** bump rust in the docker-minor group ([#149](https://github.com/jobtrek/sw/issues/149)) ([36edc29](https://github.com/jobtrek/sw/commit/36edc29151d12227b8a57b81970e2eaef70f1703))
* **deps:** bump serde from 1.0.226 to 1.0.228 ([#151](https://github.com/jobtrek/sw/issues/151)) ([073dd0c](https://github.com/jobtrek/sw/commit/073dd0c2fd0b01b7717177dc27c214a4e01abb72))


### Continuous Integration

* remove not necessary line for clippy and fmt ([#153](https://github.com/jobtrek/sw/issues/153)) ([8791c8c](https://github.com/jobtrek/sw/commit/8791c8c05b1dde6bfef3cc09a652fb8bed1f6643))

## [0.4.15](https://github.com/jobtrek/sw/compare/v0.4.14...v0.4.15) (2025-09-23)


### Miscellaneous Chores

* **deps:** bump clap from 4.5.47 to 4.5.48 ([#147](https://github.com/jobtrek/sw/issues/147)) ([c54785a](https://github.com/jobtrek/sw/commit/c54785ad0a2c9548e633002c11326f44773aa10f))
* **deps:** bump serde from 1.0.223 to 1.0.226 ([#148](https://github.com/jobtrek/sw/issues/148)) ([2c8d8c1](https://github.com/jobtrek/sw/commit/2c8d8c15b94ab0f9850009bc1360eadc8858e181))

## [0.4.14](https://github.com/jobtrek/sw/compare/v0.4.13...v0.4.14) (2025-09-15)


### Miscellaneous Chores

* **deps:** bump clap from 4.5.46 to 4.5.47 ([#143](https://github.com/jobtrek/sw/issues/143)) ([43752d8](https://github.com/jobtrek/sw/commit/43752d830d1e9623b627f8e581551cdd225a7419))
* **deps:** bump serde from 1.0.219 to 1.0.223 ([#146](https://github.com/jobtrek/sw/issues/146)) ([cd299c9](https://github.com/jobtrek/sw/commit/cd299c9f724d43892f0d6f3f70115c1f64de0412))
* **deps:** bump serde_json from 1.0.143 to 1.0.145 ([#145](https://github.com/jobtrek/sw/issues/145)) ([ba92798](https://github.com/jobtrek/sw/commit/ba9279878fbfd53135b61fa3d0ee413222d8c6da))

## [0.4.13](https://github.com/jobtrek/sw/compare/v0.4.12...v0.4.13) (2025-09-03)


### Miscellaneous Chores

* **deps:** bump clap from 4.5.45 to 4.5.46 ([#141](https://github.com/jobtrek/sw/issues/141)) ([62ece30](https://github.com/jobtrek/sw/commit/62ece302a9fe37bf0807be82edcd89b394c5b72d))

## [0.4.12](https://github.com/jobtrek/sw/compare/v0.4.11...v0.4.12) (2025-08-26)


### Miscellaneous Chores

* **deps:** bump actions/checkout from 4 to 5 ([#137](https://github.com/jobtrek/sw/issues/137)) ([574b253](https://github.com/jobtrek/sw/commit/574b2536c0fac6539b99b9fd5ebfc05a0bece977))
* **deps:** bump clap from 4.5.42 to 4.5.43 ([#135](https://github.com/jobtrek/sw/issues/135)) ([73ff4e3](https://github.com/jobtrek/sw/commit/73ff4e38a0bf11b874bbc87fba4df4717d206a54))
* **deps:** bump clap from 4.5.43 to 4.5.45 ([#140](https://github.com/jobtrek/sw/issues/140)) ([63876ff](https://github.com/jobtrek/sw/commit/63876ffc3464cf0a3cc898b4dccb37b57e3e4b59))
* **deps:** bump rayon from 1.10.0 to 1.11.0 ([#139](https://github.com/jobtrek/sw/issues/139)) ([60d38d9](https://github.com/jobtrek/sw/commit/60d38d9dc03ed04dec19089bf8fbbda8daaae418))
* **deps:** bump rust in the docker-minor group ([#134](https://github.com/jobtrek/sw/issues/134)) ([66e1e5a](https://github.com/jobtrek/sw/commit/66e1e5a6ca4cfe251b313bfcb355b956883ff99d))
* **deps:** bump serde_json from 1.0.142 to 1.0.143 ([#138](https://github.com/jobtrek/sw/issues/138)) ([b335827](https://github.com/jobtrek/sw/commit/b33582745670c10c58e48856de629b11631c029b))

## [0.4.11](https://github.com/jobtrek/sw/compare/v0.4.10...v0.4.11) (2025-08-05)


### Miscellaneous Chores

* **deps:** bump clap from 4.5.41 to 4.5.42 ([#131](https://github.com/jobtrek/sw/issues/131)) ([f51c744](https://github.com/jobtrek/sw/commit/f51c74404440bc69f7e60eecb7003b653e90266a))
* **deps:** bump serde_json from 1.0.140 to 1.0.142 ([#132](https://github.com/jobtrek/sw/issues/132)) ([6f27b5d](https://github.com/jobtrek/sw/commit/6f27b5d9f1fd3eb39ed7b3217ab2a21dba57b85d))

## [0.4.10](https://github.com/jobtrek/sw/compare/v0.4.9...v0.4.10) (2025-08-04)


### Features

* add command line arguments ([6611550](https://github.com/jobtrek/sw/commit/66115502b4d35dea8caf722a6f98084ecc81e84f))
* add files with the expectated result of the tests ([#20](https://github.com/jobtrek/sw/issues/20)) ([63b2457](https://github.com/jobtrek/sw/commit/63b24575cea23b3f6d9718368867ee050096534c))
* add java language ([#24](https://github.com/jobtrek/sw/issues/24)) ([0266bd9](https://github.com/jobtrek/sw/commit/0266bd92947cc3b885a7fe3d57bfa29f10ebf02d))
* add php language ([#33](https://github.com/jobtrek/sw/issues/33)) ([b00b595](https://github.com/jobtrek/sw/commit/b00b5952310f004e099656d8b3ebd924bc63c837))
* add support for the Javascript language ([#12](https://github.com/jobtrek/sw/issues/12)) ([9559804](https://github.com/jobtrek/sw/commit/955980448b8b22d9d49969f05dd03d38311529be))
* add test expectation files ([63b2457](https://github.com/jobtrek/sw/commit/63b24575cea23b3f6d9718368867ee050096534c))
* add toto!() macro to each functions ([9fbb77d](https://github.com/jobtrek/sw/commit/9fbb77dc754b058cda3d7ab052ca9fc3d3076fcd))
* add typescript ([#17](https://github.com/jobtrek/sw/issues/17)) ([fca07e0](https://github.com/jobtrek/sw/commit/fca07e0afa006bbed1d9f68b093cf880b9bb84f3))
* allow user to chose the file type ([6611550](https://github.com/jobtrek/sw/commit/66115502b4d35dea8caf722a6f98084ecc81e84f))
* allow user to chose to disable feedback ([6611550](https://github.com/jobtrek/sw/commit/66115502b4d35dea8caf722a6f98084ecc81e84f))
* create a developement version of the dockerfile ([9fbb77d](https://github.com/jobtrek/sw/commit/9fbb77dc754b058cda3d7ab052ca9fc3d3076fcd))
* ensure that the given extensions are all valid ([9559804](https://github.com/jobtrek/sw/commit/955980448b8b22d9d49969f05dd03d38311529be))
* get the lines number of the lines to remove ([9fbb77d](https://github.com/jobtrek/sw/commit/9fbb77dc754b058cda3d7ab052ca9fc3d3076fcd))
* get the list of files ending with an extension in a path ([9fbb77d](https://github.com/jobtrek/sw/commit/9fbb77dc754b058cda3d7ab052ca9fc3d3076fcd))
* indent replacing text according to the comment's indentation ([9fbb77d](https://github.com/jobtrek/sw/commit/9fbb77dc754b058cda3d7ab052ca9fc3d3076fcd))
* new wipe based ont token ([#105](https://github.com/jobtrek/sw/issues/105)) ([7ccc70c](https://github.com/jobtrek/sw/commit/7ccc70c010d0ef01165c71fa6814c3ffde99835a))
* only write "todo!()" when working with rust files ([9559804](https://github.com/jobtrek/sw/commit/955980448b8b22d9d49969f05dd03d38311529be))
* parse json output from ast-grep ([9fbb77d](https://github.com/jobtrek/sw/commit/9fbb77dc754b058cda3d7ab052ca9fc3d3076fcd))
* remove code after comments in rust ([#8](https://github.com/jobtrek/sw/issues/8)) ([9fbb77d](https://github.com/jobtrek/sw/commit/9fbb77dc754b058cda3d7ab052ca9fc3d3076fcd))
* remove the wanted code from the file ([9fbb77d](https://github.com/jobtrek/sw/commit/9fbb77dc754b058cda3d7ab052ca9fc3d3076fcd))
* support javascript files ([9559804](https://github.com/jobtrek/sw/commit/955980448b8b22d9d49969f05dd03d38311529be))
* update dockerfile ([9fbb77d](https://github.com/jobtrek/sw/commit/9fbb77dc754b058cda3d7ab052ca9fc3d3076fcd))
* upgrade error messages ([b6e7bfc](https://github.com/jobtrek/sw/commit/b6e7bfc77ef16749f8f397f90421245d497f1441))
* upgrade path selection ([#22](https://github.com/jobtrek/sw/issues/22)) ([e421db0](https://github.com/jobtrek/sw/commit/e421db002b35470ad7f795c796675e4f94fc942b))
* use ast-grep to get block with wanted comment ([9fbb77d](https://github.com/jobtrek/sw/commit/9fbb77dc754b058cda3d7ab052ca9fc3d3076fcd))
* use clap to chose path to run sw from ([6611550](https://github.com/jobtrek/sw/commit/66115502b4d35dea8caf722a6f98084ecc81e84f))


### Bug Fixes

* select definition scope with the other scopes ([9fbb77d](https://github.com/jobtrek/sw/commit/9fbb77dc754b058cda3d7ab052ca9fc3d3076fcd))


### Tests

* add the most relevent test done to the project files ([9fbb77d](https://github.com/jobtrek/sw/commit/9fbb77dc754b058cda3d7ab052ca9fc3d3076fcd))
* patch output files ([#106](https://github.com/jobtrek/sw/issues/106)) ([9a7ac1a](https://github.com/jobtrek/sw/commit/9a7ac1a92f8a566cce64ac932458616ea93090fc))
* update test files to new wipe token ([7ccc70c](https://github.com/jobtrek/sw/commit/7ccc70c010d0ef01165c71fa6814c3ffde99835a))


### Performance Improvements

* make get_files and run_command 1 line long ([9fbb77d](https://github.com/jobtrek/sw/commit/9fbb77dc754b058cda3d7ab052ca9fc3d3076fcd))
* remove variable to querry data directly when usefull ([9fbb77d](https://github.com/jobtrek/sw/commit/9fbb77dc754b058cda3d7ab052ca9fc3d3076fcd))


### Miscellaneous Chores

* .idea folder in gitignore ([b2fc4dc](https://github.com/jobtrek/sw/commit/b2fc4dc62df7f0701b2d551b2d8969905cb5c873))
* add license ([#15](https://github.com/jobtrek/sw/issues/15)) ([379f21e](https://github.com/jobtrek/sw/commit/379f21eb44db847eda9aeec7abbef1ba976de936))
* cargo initialisation ([6c152b5](https://github.com/jobtrek/sw/commit/6c152b5c078f87f521fd83e076ca537b9b88ecce))
* **deps:** bump alpine from 3.21 to 3.22 in the docker-minor group ([#122](https://github.com/jobtrek/sw/issues/122)) ([ac47908](https://github.com/jobtrek/sw/commit/ac47908c84da4be22eeef26102059ed7fad7c669))
* **deps:** bump clap from 4.5.15 to 4.5.16 ([#50](https://github.com/jobtrek/sw/issues/50)) ([2ff478a](https://github.com/jobtrek/sw/commit/2ff478a926d119b2d5295732ae8de85566e8e945))
* **deps:** bump clap from 4.5.16 to 4.5.17 ([#55](https://github.com/jobtrek/sw/issues/55)) ([b1e1961](https://github.com/jobtrek/sw/commit/b1e196133222845702a7f0510a8486d6e488e7cf))
* **deps:** bump clap from 4.5.17 to 4.5.18 ([#60](https://github.com/jobtrek/sw/issues/60)) ([91c47ee](https://github.com/jobtrek/sw/commit/91c47ee13e2d2dfb16adbcd3a60f8cc5bbdf3354))
* **deps:** bump clap from 4.5.18 to 4.5.20 ([#63](https://github.com/jobtrek/sw/issues/63)) ([4093294](https://github.com/jobtrek/sw/commit/409329481df78c69e581e554602c005a962acc95))
* **deps:** bump clap from 4.5.20 to 4.5.21 ([#72](https://github.com/jobtrek/sw/issues/72)) ([67c5f96](https://github.com/jobtrek/sw/commit/67c5f963fd8cd5dab43eb0ecd52a22f46662bab6))
* **deps:** bump clap from 4.5.21 to 4.5.23 ([#76](https://github.com/jobtrek/sw/issues/76)) ([2df72f5](https://github.com/jobtrek/sw/commit/2df72f5762e7442c39909c479bf4c88dc972a88c))
* **deps:** bump clap from 4.5.23 to 4.5.26 ([#82](https://github.com/jobtrek/sw/issues/82)) ([91de810](https://github.com/jobtrek/sw/commit/91de810ab370b68ec63998b5115685240b353791))
* **deps:** bump clap from 4.5.26 to 4.5.27 ([#87](https://github.com/jobtrek/sw/issues/87)) ([89ed616](https://github.com/jobtrek/sw/commit/89ed6163884a9bdc9ae3c038e66873aa65892155))
* **deps:** bump clap from 4.5.27 to 4.5.28 ([#90](https://github.com/jobtrek/sw/issues/90)) ([43cb733](https://github.com/jobtrek/sw/commit/43cb733ab5bcbf28a6ff965a4421faee3646b07d))
* **deps:** bump clap from 4.5.28 to 4.5.29 ([#92](https://github.com/jobtrek/sw/issues/92)) ([7e3a43b](https://github.com/jobtrek/sw/commit/7e3a43ba2d90ae6752408de9601aa9042bf273c1))
* **deps:** bump clap from 4.5.29 to 4.5.31 ([#99](https://github.com/jobtrek/sw/issues/99)) ([317c211](https://github.com/jobtrek/sw/commit/317c211bf8577fadf747076375cc6952095b3150))
* **deps:** bump clap from 4.5.31 to 4.5.32 ([#104](https://github.com/jobtrek/sw/issues/104)) ([b6e33e6](https://github.com/jobtrek/sw/commit/b6e33e6a36f36cd882fa06fa2da614bafd6bc434))
* **deps:** bump clap from 4.5.32 to 4.5.34 ([#109](https://github.com/jobtrek/sw/issues/109)) ([d4efbf4](https://github.com/jobtrek/sw/commit/d4efbf4737bb468449fa7af8ea5d766442534a2a))
* **deps:** bump clap from 4.5.35 to 4.5.36 ([#115](https://github.com/jobtrek/sw/issues/115)) ([7c5200a](https://github.com/jobtrek/sw/commit/7c5200a1fe6f0335ce241628eedb80e79a94ceb2))
* **deps:** bump clap from 4.5.36 to 4.5.38 ([#118](https://github.com/jobtrek/sw/issues/118)) ([c8f14db](https://github.com/jobtrek/sw/commit/c8f14dbf2d0ff22ff727b69a5549daac562aa2c9))
* **deps:** bump clap from 4.5.38 to 4.5.39 ([#123](https://github.com/jobtrek/sw/issues/123)) ([7c46994](https://github.com/jobtrek/sw/commit/7c46994d60e95faaafd2011f84d1a23fdbcdfb21))
* **deps:** bump clap from 4.5.39 to 4.5.40 ([#125](https://github.com/jobtrek/sw/issues/125)) ([d308f6d](https://github.com/jobtrek/sw/commit/d308f6d75edc2739040b7d98b60fafb0f687697b))
* **deps:** bump clap from 4.5.40 to 4.5.41 ([#130](https://github.com/jobtrek/sw/issues/130)) ([90e5276](https://github.com/jobtrek/sw/commit/90e52760599a6835e97d591ca3f49703378af8dd))
* **deps:** bump clap from 4.5.7 to 4.5.8 ([#18](https://github.com/jobtrek/sw/issues/18)) ([6c344cb](https://github.com/jobtrek/sw/commit/6c344cb93846815f52e3d3b0d57e4ed5f1fc3703))
* **deps:** bump clap from 4.5.8 to 4.5.15 ([#42](https://github.com/jobtrek/sw/issues/42)) ([dbf25b2](https://github.com/jobtrek/sw/commit/dbf25b25f6488ea35f7f76538157bfad55705ad9))
* **deps:** bump rust in the docker-minor group ([#112](https://github.com/jobtrek/sw/issues/112)) ([20afad3](https://github.com/jobtrek/sw/commit/20afad3be30c9177e2392233516b65bc8b81c355))
* **deps:** bump rust in the docker-minor group ([#127](https://github.com/jobtrek/sw/issues/127)) ([381b93a](https://github.com/jobtrek/sw/commit/381b93ab236a006e2d0f045461c7490121b09ae5))
* **deps:** bump rust in the docker-minor group ([#47](https://github.com/jobtrek/sw/issues/47)) ([21c0586](https://github.com/jobtrek/sw/commit/21c0586568f548aa42109d32300d2c8484038e6f))
* **deps:** bump rust in the docker-minor group ([#58](https://github.com/jobtrek/sw/issues/58)) ([c4046f0](https://github.com/jobtrek/sw/commit/c4046f0b70b6a1ffd698f4d0ea2eed2594adb3c5))
* **deps:** bump rust in the docker-minor group ([#65](https://github.com/jobtrek/sw/issues/65)) ([a809c5e](https://github.com/jobtrek/sw/commit/a809c5e70306ad4fd47def9dd0730e671f7c6312))
* **deps:** bump rust in the docker-minor group ([#84](https://github.com/jobtrek/sw/issues/84)) ([0d68dee](https://github.com/jobtrek/sw/commit/0d68deee3fd584c7e84109774b8704845f3ad8cf))
* **deps:** bump rust in the docker-minor group ([#97](https://github.com/jobtrek/sw/issues/97)) ([978b779](https://github.com/jobtrek/sw/commit/978b779bfb22f2622ceebe96ead69ddb695aa57c))
* **deps:** bump serde from 1.0.203 to 1.0.204 ([#27](https://github.com/jobtrek/sw/issues/27)) ([9da3f13](https://github.com/jobtrek/sw/commit/9da3f1319a08f6e175d5d0d67a288e8481c408d0))
* **deps:** bump serde from 1.0.204 to 1.0.206 ([#43](https://github.com/jobtrek/sw/issues/43)) ([311f920](https://github.com/jobtrek/sw/commit/311f9203eaf4786b75f4a4f4f4a76327cd0ae2a2))
* **deps:** bump serde from 1.0.206 to 1.0.208 ([#49](https://github.com/jobtrek/sw/issues/49)) ([53f16fc](https://github.com/jobtrek/sw/commit/53f16fcf3f92d41a21352f7bc4879743155530ea))
* **deps:** bump serde from 1.0.208 to 1.0.209 ([#52](https://github.com/jobtrek/sw/issues/52)) ([2680c42](https://github.com/jobtrek/sw/commit/2680c420485614367cb6f89dd98d16ddd59fb4c2))
* **deps:** bump serde from 1.0.209 to 1.0.210 ([#57](https://github.com/jobtrek/sw/issues/57)) ([fa52731](https://github.com/jobtrek/sw/commit/fa527316165756ebbfa0da4315814acf7e122da6))
* **deps:** bump serde from 1.0.210 to 1.0.213 ([#67](https://github.com/jobtrek/sw/issues/67)) ([817b6e0](https://github.com/jobtrek/sw/commit/817b6e0604086d186fa48b0795fa4e03d47a21d6))
* **deps:** bump serde from 1.0.213 to 1.0.214 ([#69](https://github.com/jobtrek/sw/issues/69)) ([847e8a0](https://github.com/jobtrek/sw/commit/847e8a0f00483a277f2a771f63aae4201afab26b))
* **deps:** bump serde from 1.0.214 to 1.0.215 ([#71](https://github.com/jobtrek/sw/issues/71)) ([0f464e0](https://github.com/jobtrek/sw/commit/0f464e082ee196685c3e703fca1802eb7596dc7d))
* **deps:** bump serde from 1.0.215 to 1.0.216 ([#77](https://github.com/jobtrek/sw/issues/77)) ([fa1a85c](https://github.com/jobtrek/sw/commit/fa1a85c2662a9c1b9177d3c99e687bdf4e0cc89c))
* **deps:** bump serde from 1.0.216 to 1.0.217 ([#80](https://github.com/jobtrek/sw/issues/80)) ([a44036e](https://github.com/jobtrek/sw/commit/a44036e78a1e4bd5814e4818ad240168d1aff0f8))
* **deps:** bump serde from 1.0.217 to 1.0.218 ([#96](https://github.com/jobtrek/sw/issues/96)) ([b7296b8](https://github.com/jobtrek/sw/commit/b7296b8bbaa416f82f4658e5e5a3326e930aefef))
* **deps:** bump serde from 1.0.218 to 1.0.219 ([#102](https://github.com/jobtrek/sw/issues/102)) ([00ba384](https://github.com/jobtrek/sw/commit/00ba38491591bcc713cacd44cdf1f4b9fea6ce20))
* **deps:** bump serde_json from 1.0.117 to 1.0.119 ([#19](https://github.com/jobtrek/sw/issues/19)) ([7ba4e86](https://github.com/jobtrek/sw/commit/7ba4e8670e341b2d53d8cfd8e37de53afc019539))
* **deps:** bump serde_json from 1.0.119 to 1.0.120 ([#26](https://github.com/jobtrek/sw/issues/26)) ([eab724c](https://github.com/jobtrek/sw/commit/eab724cc6a10f8aeb8359c56fd05a087d89b8dc8))
* **deps:** bump serde_json from 1.0.120 to 1.0.122 ([#40](https://github.com/jobtrek/sw/issues/40)) ([34cb1cc](https://github.com/jobtrek/sw/commit/34cb1cc3b1f95a01e9d050c03a7f9b0618fcb8ab))
* **deps:** bump serde_json from 1.0.122 to 1.0.124 ([#46](https://github.com/jobtrek/sw/issues/46)) ([90f82a5](https://github.com/jobtrek/sw/commit/90f82a5e1dbfca6db78e46f210da884ce8a60d9a))
* **deps:** bump serde_json from 1.0.124 to 1.0.125 ([#48](https://github.com/jobtrek/sw/issues/48)) ([42a8d51](https://github.com/jobtrek/sw/commit/42a8d51bc27ce3fcfe4a6976bd3e1dd48476c1e2))
* **deps:** bump serde_json from 1.0.125 to 1.0.127 ([#53](https://github.com/jobtrek/sw/issues/53)) ([c3a5753](https://github.com/jobtrek/sw/commit/c3a57530f35e0f2591c4f96d8c0e3949a301bb1b))
* **deps:** bump serde_json from 1.0.127 to 1.0.128 ([#56](https://github.com/jobtrek/sw/issues/56)) ([c5ea8b5](https://github.com/jobtrek/sw/commit/c5ea8b544e2a5899dda356981983d220f0c93bc4))
* **deps:** bump serde_json from 1.0.128 to 1.0.132 ([#64](https://github.com/jobtrek/sw/issues/64)) ([9f37361](https://github.com/jobtrek/sw/commit/9f373615f8c3019e74df0d1f1237bbca8f6e91ca))
* **deps:** bump serde_json from 1.0.132 to 1.0.133 ([#70](https://github.com/jobtrek/sw/issues/70)) ([4788301](https://github.com/jobtrek/sw/commit/4788301ac854b291364a8abb25910e27c5fd94bd))
* **deps:** bump serde_json from 1.0.133 to 1.0.134 ([#79](https://github.com/jobtrek/sw/issues/79)) ([d67c737](https://github.com/jobtrek/sw/commit/d67c7379799d2632f5efc3247128894146b2c7ff))
* **deps:** bump serde_json from 1.0.134 to 1.0.135 ([#83](https://github.com/jobtrek/sw/issues/83)) ([cf97044](https://github.com/jobtrek/sw/commit/cf970448d3e281cf76e0048ba251e479f0053ea7))
* **deps:** bump serde_json from 1.0.135 to 1.0.138 ([#88](https://github.com/jobtrek/sw/issues/88)) ([26d3876](https://github.com/jobtrek/sw/commit/26d3876c92ee9022f0abe021f6dcbd6a91423e01))
* **deps:** bump serde_json from 1.0.138 to 1.0.139 ([#95](https://github.com/jobtrek/sw/issues/95)) ([ce685b7](https://github.com/jobtrek/sw/commit/ce685b7656ef7d3e8f2b03e320e1ef77b299ccc1))
* **deps:** bump serde_json from 1.0.139 to 1.0.140 ([#100](https://github.com/jobtrek/sw/issues/100)) ([6ff67b4](https://github.com/jobtrek/sw/commit/6ff67b412e93fabbe286c2afec3fb1b857e2923c))
* **deps:** bump the docker-minor group with 2 updates ([#75](https://github.com/jobtrek/sw/issues/75)) ([1549224](https://github.com/jobtrek/sw/commit/1549224f9c4f3aaaa608c4369bf5952acdbb4cbb))
* **main:** release 0.1.1 ([#4](https://github.com/jobtrek/sw/issues/4)) ([787d3d2](https://github.com/jobtrek/sw/commit/787d3d28393da5699318a9308204403767aea560))
* **main:** release 0.1.2 ([#6](https://github.com/jobtrek/sw/issues/6)) ([5e7353a](https://github.com/jobtrek/sw/commit/5e7353ae73cf6cfbdfcc624127f01f2bde3c9b29))
* **main:** release 0.1.3 ([#14](https://github.com/jobtrek/sw/issues/14)) ([96d5906](https://github.com/jobtrek/sw/commit/96d59064eeaf15cf356d5301144d9d6643cd2859))
* **main:** release 0.1.4 ([#16](https://github.com/jobtrek/sw/issues/16)) ([cd5453f](https://github.com/jobtrek/sw/commit/cd5453faadf1e2475988144fdf4b15c8c2b3b4e4))
* **main:** release 0.1.5 ([#23](https://github.com/jobtrek/sw/issues/23)) ([b28c938](https://github.com/jobtrek/sw/commit/b28c93821389526170e48dfe52d0f7c6c8d886df))
* **main:** release 0.2.0 ([#28](https://github.com/jobtrek/sw/issues/28)) ([3ed4a6e](https://github.com/jobtrek/sw/commit/3ed4a6eb690ceb0cc085457828180f7fd41fe9a9))
* **main:** release 0.2.1 ([#45](https://github.com/jobtrek/sw/issues/45)) ([029a103](https://github.com/jobtrek/sw/commit/029a103a60588661bcad8b8fe1c1fad369adc995))
* **main:** release 0.2.2 ([#51](https://github.com/jobtrek/sw/issues/51)) ([e17c0a9](https://github.com/jobtrek/sw/commit/e17c0a95bd04eb0f123ebe43559ce4f0080fe2d0))
* **main:** release 0.2.3 ([#54](https://github.com/jobtrek/sw/issues/54)) ([5f9f9d3](https://github.com/jobtrek/sw/commit/5f9f9d3b53c7ef7fcd46d27f551c335cfd35cc07))
* **main:** release 0.2.4 ([#59](https://github.com/jobtrek/sw/issues/59)) ([1229392](https://github.com/jobtrek/sw/commit/1229392eec0cf27b4d949b4dcc8c691dc07a34cb))
* **main:** release 0.2.5 ([#61](https://github.com/jobtrek/sw/issues/61)) ([54d9565](https://github.com/jobtrek/sw/commit/54d9565bf877e4d84af149cd7b23b7c0a0968c06))
* **main:** release 0.2.6 ([#66](https://github.com/jobtrek/sw/issues/66)) ([2d35b12](https://github.com/jobtrek/sw/commit/2d35b12d19021dc09766db81f5da1a43d7ad93da))
* **main:** release 0.2.7 ([#68](https://github.com/jobtrek/sw/issues/68)) ([78ea3ba](https://github.com/jobtrek/sw/commit/78ea3ba22d11c942803ed6cfce903636eb137374))
* **main:** release 0.3.0 ([#74](https://github.com/jobtrek/sw/issues/74)) ([02c4f07](https://github.com/jobtrek/sw/commit/02c4f078bd12246fa696d8617274847b396c8e28))
* **main:** release 0.3.1 ([#78](https://github.com/jobtrek/sw/issues/78)) ([0f67bbe](https://github.com/jobtrek/sw/commit/0f67bbe0b7c20110ea60671225c2aa4c51906de3))
* **main:** release 0.3.2 ([#81](https://github.com/jobtrek/sw/issues/81)) ([0cc9b5d](https://github.com/jobtrek/sw/commit/0cc9b5d2e6c41cd32c002b7e9c65ef3ea2057835))
* **main:** release 0.3.3 ([#85](https://github.com/jobtrek/sw/issues/85)) ([e7fdced](https://github.com/jobtrek/sw/commit/e7fdced30987c2862c971340a060981dc249962e))
* **main:** release 0.3.4 ([#89](https://github.com/jobtrek/sw/issues/89)) ([42908cc](https://github.com/jobtrek/sw/commit/42908cc46c1ad519c6c36af8dbf2471b533a55b1))
* **main:** release 0.3.5 ([#91](https://github.com/jobtrek/sw/issues/91)) ([66bc896](https://github.com/jobtrek/sw/commit/66bc896e9540df717995b3333c169de140f53196))
* **main:** release 0.3.6 ([#93](https://github.com/jobtrek/sw/issues/93)) ([163f654](https://github.com/jobtrek/sw/commit/163f6549014511e8153aaae6145d0dbaaf830f54))
* **main:** release 0.3.7 ([#98](https://github.com/jobtrek/sw/issues/98)) ([2e32a60](https://github.com/jobtrek/sw/commit/2e32a60a015abe5a0d0fe3d7a6fa9c78209fe56e))
* **main:** release 0.3.8 ([#101](https://github.com/jobtrek/sw/issues/101)) ([cac89e9](https://github.com/jobtrek/sw/commit/cac89e922873d81edc7c57cf5555344bce17890a))
* **main:** release 0.4.0 ([#103](https://github.com/jobtrek/sw/issues/103)) ([3e553f3](https://github.com/jobtrek/sw/commit/3e553f31d61543e5f19ac58b38029eb45172e943))
* **main:** release 0.4.1 ([#107](https://github.com/jobtrek/sw/issues/107)) ([293c196](https://github.com/jobtrek/sw/commit/293c1968d2264974c4854be067f3d0c7bdb61b18))
* **main:** release 0.4.2 ([#111](https://github.com/jobtrek/sw/issues/111)) ([db5c1e5](https://github.com/jobtrek/sw/commit/db5c1e594c5bd13f6eb853d262fa6e3b6929e10d))
* **main:** release 0.4.3 ([#113](https://github.com/jobtrek/sw/issues/113)) ([9b52721](https://github.com/jobtrek/sw/commit/9b527216c4f145f9a8cbc0dae87512d6557bb622))
* **main:** release 0.4.4 ([#116](https://github.com/jobtrek/sw/issues/116)) ([6e23550](https://github.com/jobtrek/sw/commit/6e235509cec786735c5ead81ac03f9b6d94050d3))
* **main:** release 0.4.5 ([#119](https://github.com/jobtrek/sw/issues/119)) ([80dccd3](https://github.com/jobtrek/sw/commit/80dccd3a73bd7506056c2f68a6c9ec54ed352546))
* **main:** release 0.4.6 ([#121](https://github.com/jobtrek/sw/issues/121)) ([95228b6](https://github.com/jobtrek/sw/commit/95228b67cc8d4c0ac54ec58716ac97d07052ecb9))
* **main:** release 0.4.7 ([#124](https://github.com/jobtrek/sw/issues/124)) ([d77550e](https://github.com/jobtrek/sw/commit/d77550ef75b5a406b5a4bb8790282a67a7f315d4))
* **main:** release 0.4.8 ([#126](https://github.com/jobtrek/sw/issues/126)) ([ee36864](https://github.com/jobtrek/sw/commit/ee36864a3d42a6b632c69e06fe67ab506783bea8))
* **main:** release 0.4.9 ([#128](https://github.com/jobtrek/sw/issues/128)) ([1ce4ca3](https://github.com/jobtrek/sw/commit/1ce4ca3a13f1b375382e1919cfc3def9c3c21466))
* release 0.4.10 ([b9f9af9](https://github.com/jobtrek/sw/commit/b9f9af902e0e1476fd8b4f5b8191a93380dc97fb))
* remove prerelease from release please config ([#34](https://github.com/jobtrek/sw/issues/34)) ([6205fd8](https://github.com/jobtrek/sw/commit/6205fd878a6807186f2b1172d3da2482430abc35))
* rm unused deps ([#110](https://github.com/jobtrek/sw/issues/110)) ([8611691](https://github.com/jobtrek/sw/commit/8611691e44863bea999488164fa27478c65c790a))
* setup codeowners ([#5](https://github.com/jobtrek/sw/issues/5)) ([3df853c](https://github.com/jobtrek/sw/commit/3df853c37d70662dda2df5567f1270d386b0e8c3))
* update to rust 1.87 and deps bump minors ([#120](https://github.com/jobtrek/sw/issues/120)) ([8300860](https://github.com/jobtrek/sw/commit/830086073da45ebab5012489e5be2ac517878ec9))


### Documentation

* fix rust extension file ([#7](https://github.com/jobtrek/sw/issues/7)) ([838e182](https://github.com/jobtrek/sw/commit/838e182fde84e47679c8d276ca7956a0b24bb018))
* initial readme ([8e9da77](https://github.com/jobtrek/sw/commit/8e9da77c1fd3296ef2d19789125c017679cf9995))
* sw defaults description ([3cf6c8d](https://github.com/jobtrek/sw/commit/3cf6c8d5db5ba38ea961ce5780ed217387faebd1))
* sw defaults description ([0b75e6b](https://github.com/jobtrek/sw/commit/0b75e6b6e59ca1341036463a988ca4e9500474b8))


### Refactors

* addapt check_paths_exist function to return error instead of panicking ([b6e7bfc](https://github.com/jobtrek/sw/commit/b6e7bfc77ef16749f8f397f90421245d497f1441))
* addapt get_files_per_extension function to return error instead of panicking ([b6e7bfc](https://github.com/jobtrek/sw/commit/b6e7bfc77ef16749f8f397f90421245d497f1441))
* addapt get_removable_parts function to return error instead of panicking ([b6e7bfc](https://github.com/jobtrek/sw/commit/b6e7bfc77ef16749f8f397f90421245d497f1441))
* addapt remove_parts function to return error instead of panicking ([b6e7bfc](https://github.com/jobtrek/sw/commit/b6e7bfc77ef16749f8f397f90421245d497f1441))
* addapt run_command function to return error instead of panicking ([b6e7bfc](https://github.com/jobtrek/sw/commit/b6e7bfc77ef16749f8f397f90421245d497f1441))
* make all structure field snake_case ([9fbb77d](https://github.com/jobtrek/sw/commit/9fbb77dc754b058cda3d7ab052ca9fc3d3076fcd))
* move project to /etc/jobtrek/sw ([9fbb77d](https://github.com/jobtrek/sw/commit/9fbb77dc754b058cda3d7ab052ca9fc3d3076fcd))
* move test and expected result in same folder ([63b2457](https://github.com/jobtrek/sw/commit/63b24575cea23b3f6d9718368867ee050096534c))
* run cargo fmt and cargo clippy ([9fbb77d](https://github.com/jobtrek/sw/commit/9fbb77dc754b058cda3d7ab052ca9fc3d3076fcd))
* update ast grep parse error to be a more precise type ([b6e7bfc](https://github.com/jobtrek/sw/commit/b6e7bfc77ef16749f8f397f90421245d497f1441))
* use an enum to recognize extensions instead of testing it in main() ([#21](https://github.com/jobtrek/sw/issues/21)) ([8ad79eb](https://github.com/jobtrek/sw/commit/8ad79eb68b0245879c5ffc7176855883f8d418a8))
* use rayon for parallel file wipe ([7ccc70c](https://github.com/jobtrek/sw/commit/7ccc70c010d0ef01165c71fa6814c3ffde99835a))
* use Result to handle errors instead of panic, unwrap, expect and exit ([#35](https://github.com/jobtrek/sw/issues/35)) ([b6e7bfc](https://github.com/jobtrek/sw/commit/b6e7bfc77ef16749f8f397f90421245d497f1441))
* use structstruck to nest structures to look like the json ([9fbb77d](https://github.com/jobtrek/sw/commit/9fbb77dc754b058cda3d7ab052ca9fc3d3076fcd))


### Build System

* remove ast-grep in dockerfile ([7ccc70c](https://github.com/jobtrek/sw/commit/7ccc70c010d0ef01165c71fa6814c3ffde99835a))


### Continuous Integration

* add clippy verification tu action ([b2fc4dc](https://github.com/jobtrek/sw/commit/b2fc4dc62df7f0701b2d551b2d8969905cb5c873))
* add concurrency group to avoid concurrent running ([#11](https://github.com/jobtrek/sw/issues/11)) ([21ff0d3](https://github.com/jobtrek/sw/commit/21ff0d3d864809fe3822a10c1e85c64dc70bacdc))
* add docker to dependabot ([#44](https://github.com/jobtrek/sw/issues/44)) ([5b6c5df](https://github.com/jobtrek/sw/commit/5b6c5dfb973029206a7e4a83a39d0e3d8b3cfc36))
* add release please ([#3](https://github.com/jobtrek/sw/issues/3)) ([a4222ff](https://github.com/jobtrek/sw/commit/a4222fff6017e55c5056f68aeb72b3b7827c828e))
* add release please configuration files ([a4222ff](https://github.com/jobtrek/sw/commit/a4222fff6017e55c5056f68aeb72b3b7827c828e))
* add release please releaser action ([a4222ff](https://github.com/jobtrek/sw/commit/a4222fff6017e55c5056f68aeb72b3b7827c828e))
* setup ci action ([#2](https://github.com/jobtrek/sw/issues/2)) ([b2fc4dc](https://github.com/jobtrek/sw/commit/b2fc4dc62df7f0701b2d551b2d8969905cb5c873))
* setup dependabot ([#9](https://github.com/jobtrek/sw/issues/9)) ([225f515](https://github.com/jobtrek/sw/commit/225f5150b393334a25e15ba1e2d6d7fad3ea8524))
* setup formatter and test check workflow ([b2fc4dc](https://github.com/jobtrek/sw/commit/b2fc4dc62df7f0701b2d551b2d8969905cb5c873))

## [0.4.9](https://github.com/jobtrek/sw/compare/v0.4.8...v0.4.9) (2025-07-03)


### Miscellaneous Chores

* **deps:** bump rust in the docker-minor group ([#127](https://github.com/jobtrek/sw/issues/127)) ([381b93a](https://github.com/jobtrek/sw/commit/381b93ab236a006e2d0f045461c7490121b09ae5))

## [0.4.8](https://github.com/jobtrek/sw/compare/v0.4.7...v0.4.8) (2025-06-16)


### Miscellaneous Chores

* **deps:** bump clap from 4.5.39 to 4.5.40 ([#125](https://github.com/jobtrek/sw/issues/125)) ([d308f6d](https://github.com/jobtrek/sw/commit/d308f6d75edc2739040b7d98b60fafb0f687697b))

## [0.4.7](https://github.com/jobtrek/sw/compare/v0.4.6...v0.4.7) (2025-06-02)


### Miscellaneous Chores

* **deps:** bump alpine from 3.21 to 3.22 in the docker-minor group ([#122](https://github.com/jobtrek/sw/issues/122)) ([ac47908](https://github.com/jobtrek/sw/commit/ac47908c84da4be22eeef26102059ed7fad7c669))
* **deps:** bump clap from 4.5.38 to 4.5.39 ([#123](https://github.com/jobtrek/sw/issues/123)) ([7c46994](https://github.com/jobtrek/sw/commit/7c46994d60e95faaafd2011f84d1a23fdbcdfb21))

## [0.4.6](https://github.com/jobtrek/sw/compare/v0.4.5...v0.4.6) (2025-05-19)


### Miscellaneous Chores

* update to rust 1.87 and deps bump minors ([#120](https://github.com/jobtrek/sw/issues/120)) ([8300860](https://github.com/jobtrek/sw/commit/830086073da45ebab5012489e5be2ac517878ec9))

## [0.4.5](https://github.com/jobtrek/sw/compare/v0.4.4...v0.4.5) (2025-05-13)


### Miscellaneous Chores

* **deps:** bump clap from 4.5.36 to 4.5.38 ([#118](https://github.com/jobtrek/sw/issues/118)) ([c8f14db](https://github.com/jobtrek/sw/commit/c8f14dbf2d0ff22ff727b69a5549daac562aa2c9))

## [0.4.4](https://github.com/jobtrek/sw/compare/v0.4.3...v0.4.4) (2025-04-15)


### Miscellaneous Chores

* **deps:** bump clap from 4.5.35 to 4.5.36 ([#115](https://github.com/jobtrek/sw/issues/115)) ([7c5200a](https://github.com/jobtrek/sw/commit/7c5200a1fe6f0335ce241628eedb80e79a94ceb2))

## [0.4.3](https://github.com/jobtrek/sw/compare/v0.4.2...v0.4.3) (2025-04-10)


### Miscellaneous Chores

* **deps:** bump rust in the docker-minor group ([#112](https://github.com/jobtrek/sw/issues/112)) ([20afad3](https://github.com/jobtrek/sw/commit/20afad3be30c9177e2392233516b65bc8b81c355))

## [0.4.2](https://github.com/jobtrek/sw/compare/v0.4.1...v0.4.2) (2025-04-03)


### Miscellaneous Chores

* **deps:** bump clap from 4.5.32 to 4.5.34 ([#109](https://github.com/jobtrek/sw/issues/109)) ([d4efbf4](https://github.com/jobtrek/sw/commit/d4efbf4737bb468449fa7af8ea5d766442534a2a))
* rm unused deps ([#110](https://github.com/jobtrek/sw/issues/110)) ([8611691](https://github.com/jobtrek/sw/commit/8611691e44863bea999488164fa27478c65c790a))

## [0.4.1](https://github.com/jobtrek/sw/compare/v0.4.0...v0.4.1) (2025-03-18)


### Tests

* patch output files ([#106](https://github.com/jobtrek/sw/issues/106)) ([9a7ac1a](https://github.com/jobtrek/sw/commit/9a7ac1a92f8a566cce64ac932458616ea93090fc))

## [0.4.0](https://github.com/jobtrek/sw/compare/v0.3.8...v0.4.0) (2025-03-18)


### Features

* new wipe based ont token ([#105](https://github.com/jobtrek/sw/issues/105)) ([7ccc70c](https://github.com/jobtrek/sw/commit/7ccc70c010d0ef01165c71fa6814c3ffde99835a))


### Tests

* update test files to new wipe token ([7ccc70c](https://github.com/jobtrek/sw/commit/7ccc70c010d0ef01165c71fa6814c3ffde99835a))


### Miscellaneous Chores

* **deps:** bump clap from 4.5.31 to 4.5.32 ([#104](https://github.com/jobtrek/sw/issues/104)) ([b6e33e6](https://github.com/jobtrek/sw/commit/b6e33e6a36f36cd882fa06fa2da614bafd6bc434))
* **deps:** bump serde from 1.0.218 to 1.0.219 ([#102](https://github.com/jobtrek/sw/issues/102)) ([00ba384](https://github.com/jobtrek/sw/commit/00ba38491591bcc713cacd44cdf1f4b9fea6ce20))


### Refactors

* use rayon for parallel file wipe ([7ccc70c](https://github.com/jobtrek/sw/commit/7ccc70c010d0ef01165c71fa6814c3ffde99835a))


### Build System

* remove ast-grep in dockerfile ([7ccc70c](https://github.com/jobtrek/sw/commit/7ccc70c010d0ef01165c71fa6814c3ffde99835a))

## [0.3.8](https://github.com/jobtrek/sw/compare/v0.3.7...v0.3.8) (2025-03-04)


### Miscellaneous Chores

* **deps:** bump serde_json from 1.0.139 to 1.0.140 ([#100](https://github.com/jobtrek/sw/issues/100)) ([6ff67b4](https://github.com/jobtrek/sw/commit/6ff67b412e93fabbe286c2afec3fb1b857e2923c))

## [0.3.7](https://github.com/jobtrek/sw/compare/v0.3.6...v0.3.7) (2025-02-25)


### Miscellaneous Chores

* **deps:** bump clap from 4.5.29 to 4.5.31 ([#99](https://github.com/jobtrek/sw/issues/99)) ([317c211](https://github.com/jobtrek/sw/commit/317c211bf8577fadf747076375cc6952095b3150))
* **deps:** bump rust in the docker-minor group ([#97](https://github.com/jobtrek/sw/issues/97)) ([978b779](https://github.com/jobtrek/sw/commit/978b779bfb22f2622ceebe96ead69ddb695aa57c))
* **deps:** bump serde from 1.0.217 to 1.0.218 ([#96](https://github.com/jobtrek/sw/issues/96)) ([b7296b8](https://github.com/jobtrek/sw/commit/b7296b8bbaa416f82f4658e5e5a3326e930aefef))
* **deps:** bump serde_json from 1.0.138 to 1.0.139 ([#95](https://github.com/jobtrek/sw/issues/95)) ([ce685b7](https://github.com/jobtrek/sw/commit/ce685b7656ef7d3e8f2b03e320e1ef77b299ccc1))

## [0.3.6](https://github.com/jobtrek/sw/compare/v0.3.5...v0.3.6) (2025-02-19)


### Miscellaneous Chores

* **deps:** bump clap from 4.5.28 to 4.5.29 ([#92](https://github.com/jobtrek/sw/issues/92)) ([7e3a43b](https://github.com/jobtrek/sw/commit/7e3a43ba2d90ae6752408de9601aa9042bf273c1))

## [0.3.5](https://github.com/jobtrek/sw/compare/v0.3.4...v0.3.5) (2025-02-14)


### Miscellaneous Chores

* **deps:** bump clap from 4.5.27 to 4.5.28 ([#90](https://github.com/jobtrek/sw/issues/90)) ([43cb733](https://github.com/jobtrek/sw/commit/43cb733ab5bcbf28a6ff965a4421faee3646b07d))

## [0.3.4](https://github.com/jobtrek/sw/compare/v0.3.3...v0.3.4) (2025-02-04)


### Miscellaneous Chores

* **deps:** bump clap from 4.5.26 to 4.5.27 ([#87](https://github.com/jobtrek/sw/issues/87)) ([89ed616](https://github.com/jobtrek/sw/commit/89ed6163884a9bdc9ae3c038e66873aa65892155))
* **deps:** bump serde_json from 1.0.135 to 1.0.138 ([#88](https://github.com/jobtrek/sw/issues/88)) ([26d3876](https://github.com/jobtrek/sw/commit/26d3876c92ee9022f0abe021f6dcbd6a91423e01))

## [0.3.3](https://github.com/jobtrek/sw/compare/v0.3.2...v0.3.3) (2025-01-16)


### Miscellaneous Chores

* **deps:** bump clap from 4.5.23 to 4.5.26 ([#82](https://github.com/jobtrek/sw/issues/82)) ([91de810](https://github.com/jobtrek/sw/commit/91de810ab370b68ec63998b5115685240b353791))
* **deps:** bump rust in the docker-minor group ([#84](https://github.com/jobtrek/sw/issues/84)) ([0d68dee](https://github.com/jobtrek/sw/commit/0d68deee3fd584c7e84109774b8704845f3ad8cf))
* **deps:** bump serde_json from 1.0.134 to 1.0.135 ([#83](https://github.com/jobtrek/sw/issues/83)) ([cf97044](https://github.com/jobtrek/sw/commit/cf970448d3e281cf76e0048ba251e479f0053ea7))

## [0.3.2](https://github.com/jobtrek/sw/compare/v0.3.1...v0.3.2) (2025-01-03)


### Miscellaneous Chores

* **deps:** bump serde from 1.0.216 to 1.0.217 ([#80](https://github.com/jobtrek/sw/issues/80)) ([a44036e](https://github.com/jobtrek/sw/commit/a44036e78a1e4bd5814e4818ad240168d1aff0f8))
* **deps:** bump serde_json from 1.0.133 to 1.0.134 ([#79](https://github.com/jobtrek/sw/issues/79)) ([d67c737](https://github.com/jobtrek/sw/commit/d67c7379799d2632f5efc3247128894146b2c7ff))

## [0.3.1](https://github.com/jobtrek/sw/compare/v0.3.0...v0.3.1) (2024-12-20)


### Miscellaneous Chores

* **deps:** bump serde from 1.0.215 to 1.0.216 ([#77](https://github.com/jobtrek/sw/issues/77)) ([fa1a85c](https://github.com/jobtrek/sw/commit/fa1a85c2662a9c1b9177d3c99e687bdf4e0cc89c))

## [0.3.0](https://github.com/jobtrek/sw/compare/v0.2.7...v0.3.0) (2024-12-10)


### Features

* add php language ([#33](https://github.com/jobtrek/sw/issues/33)) ([b00b595](https://github.com/jobtrek/sw/commit/b00b5952310f004e099656d8b3ebd924bc63c837))


### Miscellaneous Chores

* **deps:** bump clap from 4.5.21 to 4.5.23 ([#76](https://github.com/jobtrek/sw/issues/76)) ([2df72f5](https://github.com/jobtrek/sw/commit/2df72f5762e7442c39909c479bf4c88dc972a88c))
* **deps:** bump the docker-minor group with 2 updates ([#75](https://github.com/jobtrek/sw/issues/75)) ([1549224](https://github.com/jobtrek/sw/commit/1549224f9c4f3aaaa608c4369bf5952acdbb4cbb))

## [0.2.7](https://github.com/jobtrek/sw/compare/v0.2.6...v0.2.7) (2024-11-19)


### Miscellaneous Chores

* **deps:** bump clap from 4.5.20 to 4.5.21 ([#72](https://github.com/jobtrek/sw/issues/72)) ([67c5f96](https://github.com/jobtrek/sw/commit/67c5f963fd8cd5dab43eb0ecd52a22f46662bab6))
* **deps:** bump serde from 1.0.210 to 1.0.213 ([#67](https://github.com/jobtrek/sw/issues/67)) ([817b6e0](https://github.com/jobtrek/sw/commit/817b6e0604086d186fa48b0795fa4e03d47a21d6))
* **deps:** bump serde from 1.0.213 to 1.0.214 ([#69](https://github.com/jobtrek/sw/issues/69)) ([847e8a0](https://github.com/jobtrek/sw/commit/847e8a0f00483a277f2a771f63aae4201afab26b))
* **deps:** bump serde from 1.0.214 to 1.0.215 ([#71](https://github.com/jobtrek/sw/issues/71)) ([0f464e0](https://github.com/jobtrek/sw/commit/0f464e082ee196685c3e703fca1802eb7596dc7d))
* **deps:** bump serde_json from 1.0.132 to 1.0.133 ([#70](https://github.com/jobtrek/sw/issues/70)) ([4788301](https://github.com/jobtrek/sw/commit/4788301ac854b291364a8abb25910e27c5fd94bd))

## [0.2.6](https://github.com/jobtrek/sw/compare/v0.2.5...v0.2.6) (2024-10-22)


### Miscellaneous Chores

* **deps:** bump clap from 4.5.18 to 4.5.20 ([#63](https://github.com/jobtrek/sw/issues/63)) ([4093294](https://github.com/jobtrek/sw/commit/409329481df78c69e581e554602c005a962acc95))
* **deps:** bump rust in the docker-minor group ([#65](https://github.com/jobtrek/sw/issues/65)) ([a809c5e](https://github.com/jobtrek/sw/commit/a809c5e70306ad4fd47def9dd0730e671f7c6312))
* **deps:** bump serde_json from 1.0.128 to 1.0.132 ([#64](https://github.com/jobtrek/sw/issues/64)) ([9f37361](https://github.com/jobtrek/sw/commit/9f373615f8c3019e74df0d1f1237bbca8f6e91ca))

## [0.2.5](https://github.com/jobtrek/sw/compare/v0.2.4...v0.2.5) (2024-09-25)


### Miscellaneous Chores

* **deps:** bump clap from 4.5.17 to 4.5.18 ([#60](https://github.com/jobtrek/sw/issues/60)) ([91c47ee](https://github.com/jobtrek/sw/commit/91c47ee13e2d2dfb16adbcd3a60f8cc5bbdf3354))

## [0.2.4](https://github.com/jobtrek/sw/compare/v0.2.3...v0.2.4) (2024-09-12)


### Miscellaneous Chores

* **deps:** bump clap from 4.5.16 to 4.5.17 ([#55](https://github.com/jobtrek/sw/issues/55)) ([b1e1961](https://github.com/jobtrek/sw/commit/b1e196133222845702a7f0510a8486d6e488e7cf))
* **deps:** bump rust in the docker-minor group ([#58](https://github.com/jobtrek/sw/issues/58)) ([c4046f0](https://github.com/jobtrek/sw/commit/c4046f0b70b6a1ffd698f4d0ea2eed2594adb3c5))
* **deps:** bump serde from 1.0.209 to 1.0.210 ([#57](https://github.com/jobtrek/sw/issues/57)) ([fa52731](https://github.com/jobtrek/sw/commit/fa527316165756ebbfa0da4315814acf7e122da6))
* **deps:** bump serde_json from 1.0.127 to 1.0.128 ([#56](https://github.com/jobtrek/sw/issues/56)) ([c5ea8b5](https://github.com/jobtrek/sw/commit/c5ea8b544e2a5899dda356981983d220f0c93bc4))

## [0.2.3](https://github.com/jobtrek/sw/compare/v0.2.2...v0.2.3) (2024-08-28)


### Miscellaneous Chores

* **deps:** bump serde from 1.0.208 to 1.0.209 ([#52](https://github.com/jobtrek/sw/issues/52)) ([2680c42](https://github.com/jobtrek/sw/commit/2680c420485614367cb6f89dd98d16ddd59fb4c2))
* **deps:** bump serde_json from 1.0.125 to 1.0.127 ([#53](https://github.com/jobtrek/sw/issues/53)) ([c3a5753](https://github.com/jobtrek/sw/commit/c3a57530f35e0f2591c4f96d8c0e3949a301bb1b))

## [0.2.2](https://github.com/jobtrek/sw/compare/v0.2.1...v0.2.2) (2024-08-19)


### Miscellaneous Chores

* **deps:** bump clap from 4.5.15 to 4.5.16 ([#50](https://github.com/jobtrek/sw/issues/50)) ([2ff478a](https://github.com/jobtrek/sw/commit/2ff478a926d119b2d5295732ae8de85566e8e945))
* **deps:** bump serde from 1.0.206 to 1.0.208 ([#49](https://github.com/jobtrek/sw/issues/49)) ([53f16fc](https://github.com/jobtrek/sw/commit/53f16fcf3f92d41a21352f7bc4879743155530ea))
* **deps:** bump serde_json from 1.0.124 to 1.0.125 ([#48](https://github.com/jobtrek/sw/issues/48)) ([42a8d51](https://github.com/jobtrek/sw/commit/42a8d51bc27ce3fcfe4a6976bd3e1dd48476c1e2))

## [0.2.1](https://github.com/jobtrek/sw/compare/v0.2.0...v0.2.1) (2024-08-12)


### Miscellaneous Chores

* **deps:** bump rust in the docker-minor group ([#47](https://github.com/jobtrek/sw/issues/47)) ([21c0586](https://github.com/jobtrek/sw/commit/21c0586568f548aa42109d32300d2c8484038e6f))
* **deps:** bump serde_json from 1.0.122 to 1.0.124 ([#46](https://github.com/jobtrek/sw/issues/46)) ([90f82a5](https://github.com/jobtrek/sw/commit/90f82a5e1dbfca6db78e46f210da884ce8a60d9a))


### Continuous Integration

* add docker to dependabot ([#44](https://github.com/jobtrek/sw/issues/44)) ([5b6c5df](https://github.com/jobtrek/sw/commit/5b6c5dfb973029206a7e4a83a39d0e3d8b3cfc36))

## [0.2.0](https://github.com/jobtrek/sw/compare/v0.1.5...v0.2.0) (2024-08-12)


### Features

* upgrade error messages ([b6e7bfc](https://github.com/jobtrek/sw/commit/b6e7bfc77ef16749f8f397f90421245d497f1441))


### Miscellaneous Chores

* **deps:** bump clap from 4.5.8 to 4.5.15 ([#42](https://github.com/jobtrek/sw/issues/42)) ([dbf25b2](https://github.com/jobtrek/sw/commit/dbf25b25f6488ea35f7f76538157bfad55705ad9))
* **deps:** bump serde from 1.0.203 to 1.0.204 ([#27](https://github.com/jobtrek/sw/issues/27)) ([9da3f13](https://github.com/jobtrek/sw/commit/9da3f1319a08f6e175d5d0d67a288e8481c408d0))
* **deps:** bump serde from 1.0.204 to 1.0.206 ([#43](https://github.com/jobtrek/sw/issues/43)) ([311f920](https://github.com/jobtrek/sw/commit/311f9203eaf4786b75f4a4f4f4a76327cd0ae2a2))
* **deps:** bump serde_json from 1.0.119 to 1.0.120 ([#26](https://github.com/jobtrek/sw/issues/26)) ([eab724c](https://github.com/jobtrek/sw/commit/eab724cc6a10f8aeb8359c56fd05a087d89b8dc8))
* **deps:** bump serde_json from 1.0.120 to 1.0.122 ([#40](https://github.com/jobtrek/sw/issues/40)) ([34cb1cc](https://github.com/jobtrek/sw/commit/34cb1cc3b1f95a01e9d050c03a7f9b0618fcb8ab))
* remove prerelease from release please config ([#34](https://github.com/jobtrek/sw/issues/34)) ([6205fd8](https://github.com/jobtrek/sw/commit/6205fd878a6807186f2b1172d3da2482430abc35))


### Refactors

* addapt check_paths_exist function to return error instead of panicking ([b6e7bfc](https://github.com/jobtrek/sw/commit/b6e7bfc77ef16749f8f397f90421245d497f1441))
* addapt get_files_per_extension function to return error instead of panicking ([b6e7bfc](https://github.com/jobtrek/sw/commit/b6e7bfc77ef16749f8f397f90421245d497f1441))
* addapt get_removable_parts function to return error instead of panicking ([b6e7bfc](https://github.com/jobtrek/sw/commit/b6e7bfc77ef16749f8f397f90421245d497f1441))
* addapt remove_parts function to return error instead of panicking ([b6e7bfc](https://github.com/jobtrek/sw/commit/b6e7bfc77ef16749f8f397f90421245d497f1441))
* addapt run_command function to return error instead of panicking ([b6e7bfc](https://github.com/jobtrek/sw/commit/b6e7bfc77ef16749f8f397f90421245d497f1441))
* update ast grep parse error to be a more precise type ([b6e7bfc](https://github.com/jobtrek/sw/commit/b6e7bfc77ef16749f8f397f90421245d497f1441))
* use Result to handle errors instead of panic, unwrap, expect and exit ([#35](https://github.com/jobtrek/sw/issues/35)) ([b6e7bfc](https://github.com/jobtrek/sw/commit/b6e7bfc77ef16749f8f397f90421245d497f1441))

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

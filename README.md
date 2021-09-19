<!--
*** Taken from Best-README-Template: 
*** https://github.com/othneildrew/Best-README-Template
*** If you have a suggestion that would make this better, 
*** please fork the repo and create a pull request
*** or simply open an issue with the tag "enhancement".
*** Thanks again! Now go create something AMAZING! :D
***
***
***
*** To avoid retyping too much info. Do a search and replace for the following:
*** github_username, repo_name, twitter_handle, email, project_title, project_description
-->



<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]



<!-- PROJECT LOGO -->
<br />
<p align="center">
  <a href="https://github.com/dbrowne/rust_server.git">
    <img src="Images/riding.png" alt="Logo" width="80" height="80">
  </a>

<h3 align="center">Rust TCP Server</h3>

  <p align="center">
    Listens on a port  and returns the message
<br />
    <br />
    <br />
    <a href="https://github.com/dbrowne/rust_server/issues">Report Bug</a>
    <br/>
    <a href="https://github.com/dbrowne/rust_server/issues">Request Feature</a>
  </p>




<!-- TABLE OF CONTENTS -->
<details open="open">
  <summary><h2 style="display: inline-block">Table of Contents</h2></summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgements">Acknowledgements</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->

## About The Project

A simple TCP server written in rust. Might be useful for some

### Built With

* [Rust](https://www.rust-lang.org/tools/install)

<!-- GETTING STARTED -->

## Getting Started

To get a local copy up and running follow these simple steps.

### Prerequisites

* rust
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

  ```

### Installation

1. Clone the repo
   ```sh
   git clone https://github.com/dbrowne/rust_server.git
   ```
2. Build and run the code
   ```sh
   cd rust_server
   cargo build --release
   ./target/release/rust_server [OPTIONS]

   ```

<!-- USAGE EXAMPLES -->

## Usage

```sh
./target/release/rust_server -p 3377 
```

Listens for messages on  port 3377 and will echo back the contents.



<!-- ROADMAP -->

## Roadmap

No real roadmap for this it is just a simple Server



<!-- CONTRIBUTING -->

## Contributing

If you really want to...

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<!-- LICENSE -->

## License

Distributed under the MIT License. See `LICENSE` for more information.



<!-- CONTACT -->

## Contact

Dwight J. Browne - [@dwightb2](https://twitter.com/dwightb2) - dwight[-dot-]browne[-at-]colorado[-dot-]edu

Project Link: [https://github.com/dbrowne/rust_server.git](https://github.com/dbrowne/rust_server.git)



<!-- ACKNOWLEDGEMENTS -->

## Acknowledgements

* [google](www.google.com)
* [stackoverflow](www.stackoverflow.com)
* [docs.rs](https://docs.rs/)

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->

[contributors-shield]: https://img.shields.io/github/contributors/dbrowne/rust_server

[contributors-url]: https://github.com/dbrowne/rust_server/graphs/contributors

[forks-shield]: https://img.shields.io/github/forks/dbrowne/rust_server

[forks-url]: https://github.com/dbrowne/rust_server/network/members

[stars-shield]: https://img.shields.io/github/stars/dbrowne/rust_server

[stars-url]: https://github.com/dbrowne/rust_server/stargazers

[issues-shield]: https://img.shields.io/github/issues/dbrowne/rust_server

[issues-url]: https://github.com/dbrowne/rust_server/issues

[license-shield]: https://img.shields.io/github/license/dbrowne/rust_server

[license-url]: https://github.com/dbrowne/rust_server/blob/master/LICENSE.txt

[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555

[linkedin-url]: https://linkedin.com/in/dwightbrowne
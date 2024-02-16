# big-numbers
a bunch of big numbers to share with links<br>
the main use cases are for <a href="https://reddit.com/r/unexpectedfactorial">r/unexpectedfactorial</a> and <a href="https://reddit.com/r/theydidthemath">r/theydidthemath</a>, since normally a lot of really big numbers show up there

in order to clone and use the project on your own device, you need to use <a href="https://github.com/microsoft/git?tab=readme-ov-file">microsoft/git</a> instead of git, and then use these commands:
```
scalar clone --no-src https://github.com/GDOR-11/big-numbers.git
cd big-numbers
git sparse-checkout set src view-number
```
*Be a bit careful though*, installing microsoft/git via homebrew somehow fucked up my neovim setup and I had to re-do it all over again. No idea what happened, but be aware of this possibility.

Downloading any numbers folder is not necessary and is of no use anymore, now the program will do everything in the github repository directly

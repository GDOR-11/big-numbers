# factorial-calculator
a bunch of big factorials to share on <a href="https://reddit.com/r/unexpectedfactorial">r/unexpectedfactorial</a>

in order to use this project while not downloading all the unnecessary factorials, I recommend using <a href="https://github.com/microsoft/git?tab=readme-ov-file">microsoft/git</a> instead of git, and then use these commands:
```
scalar clone --no-src https://github.com/GDOR-11/factorial-calculator.git
cd factorial-calculator
git sparse-checkout set src
```
*Be a bit careful though*, installing microsoft/git via homebrew somehow fucked up my neovim setup and I had to re-do it all over again. No idea what happened, but be aware of this possibility.

But, of course, by not downloading all the factorials the program will not be able to use the other factorials to skip calculations that have already been done. I will soon add a ```factorials.txt``` with the numbers that have their factorials already calculated in this repository and make the program automatically download the closest factorial to skip the biggest amount of calculation possible while not downloading *all* the factorials, but until then you will have to manually run ```git sparse-checkout add factorials/<factorial>``` in order to use other factorials

To download all the factorials you can simply run ```git sparse-checkout add factorials```

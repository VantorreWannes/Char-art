# char-art
## Convert images to text characters (and back to images)

Convert any image to characters mapping each pixel to the closest key. <br>
Uses each key's average brightness to determine which key should replace each pixel. <br>
First, download the program by downloading `char_art.exe` from the releases page (windows) or `char_art` (Linux). <br>
To run the program open any terminal and type `.\char_art.exe --path <path_to_image>` (windows) or use `./char_art.exe --path <path_to_image>` if you're on Linux. <br>
This should print the image directly in your terminal. <br>
If your image is too large to fit on your screen; Fear not. Use the built in `--shrink <u32>` option to resize your image to smaller dimensions. <br>
Most images will come out too bright if you're using a dark theme terminal with a white font. If this is the case for you use the `--darken <i32>` option to apply a darken filter to the image before processing. Alternatively use `--brighten <i32>` to brighten the image instead. <br>
<br>
If you'd rather convert the key art image to an image type immediately use the `to_image --path <path_to_output_image>` subcommand. <br>
Options you've set in the main `.\char_art.exe --path <path_to_image>` command will be applied to the output image. <br>
Set the output image's font by using the `--font <path_to_font.ttf>` and set the font size by using `--size <f32>`. <br>
If you would like a refresher on these parameters and commands you can give `--help` as an option after any command to print a quick overview of the command and its options. <br>
Have fun! `Feel free to send me any suggestions/bugs/tips that you want me to look at on discord or via a PR on github.` <br>


![minecraft](https://github.com/JokNavi/char-art/assets/109341383/d5631204-c3ac-47e3-9a48-afa8b7105ef0)
![key_image](https://github.com/JokNavi/char-art/assets/109341383/9b2a14be-d324-4a6c-93ab-b6dacf260cd5)

```
WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW
WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW
WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW
WWWWWWWWWWWWWWWWWBwL|--;:,,,,,:;-~uRWWWWWWWWWWWWWWWWWWWW
WWWWWWWWWWWWWMz!,.....,`:___:`,.....,|mWWWWWWWWWWWWWWWWW
WWWWWWWWWWWd!.....:-!|"((rrr(("~!-_,...!WWWWWWWWWWWWWWWW
WWWWWWWWW&!....,-!|~~"""(rr>>>>>>r(~-...;BWWWWWWWWWWWWWW
WWWWWWWWu,............,,`;-!"r>>>>>>(-...-WWWWWWWWWWWWWW
WWWWMt-,,,;-!!!!-----;;`,...,;!r>>>>r|`...>WWWWWWWWWWWWW
WWWv,.!SWWWWWWWWWWWWWWWWMk|....-(>>>>";...,MWWWWWWWWWWWW
WW(..(WWWWWWWWWWWWWWWWWWWWMt,...!r>>>"-,...>WWWWWWWWWWWW
W&,.,&WWWWWWWWWWWWWWWWWWWWWSr...;(>>>"-:....!>Yw&WWWWWWW
Wz..,BWWWWWWWWWWWWWWWWWWWWRSL..._">>>(-_.........,;rBWWW
WZ...FWWWWWWWWWWWWWWWWWNR#Qk>...;(>>>(-;,....-!!-:..;WWW
WW-..-uSD&&BBMMMBB&&DKHQQdwt`...!r>>>(!;,...,|"""~;..uWW
WWW"..,-rtzp3wkgSSSggkZanL!,...:">>>>(!;`....;----;..-WW
WWWWr.....,`;-!!~~~~!!-_,.....;~r>>>>(-;`....:;;;;`..`WW
WWWWL...,,................,_-|(>>>>>>(-;`....`;;;;`..,RW
WWWW"..,|"~|!!--------!!!~"(r>>>>>>>>"-;`....`;;;;:...yW
WWWW!..,">>>>>>>rrrrr>>>>>>>>>>>>>>>r~-;`....`;;;;:...!W
WWWW-..,|>>>>>>>>>>>>>>>>>>>>>>>>>>>(!;;`....:;;;;:...;W
WWWW-...-(>>>>>>>>>>>>>>>>>>>>>>>>>r~-;;`....:;;;;:...;W
WWWW!...;!(>>>>>>>>>>>>>>>>>>>>>>>r"-;;;`....:;;;;:...;W
WWWWr...:-!(>>>>>>>>>>>>>>>>>>>>r(|-;;;;,....:;;;;`...-W
WWWWz...,;-!|"r>>>>>>>>>>>>>>r("!--;;;;;,....:;;;;`...}W
WWWW&,..,_;;--!!~"""(((""""~!!--;;;;;;;;,....:;;;_,...RW
WWWWW:...:;;;;;;;----------;;;;;;;;;;;;_,....:;;;`...-WW
WWWWW-...:;;;;;;;;;;;;;;;;;;;;;;;;;;;;;_,....:;;:....nWW
```


use rand::Rng;

pub fn get_art(text: &str) -> String {
    let ascii_art = get_random_art();
    let max_width = ascii_art.lines().map(|line| line.len()).max().unwrap_or(0);
    format!(
        "{}\n{:^width$} ",
        ascii_art,
        format!("========== {} ==========", text),
        width = max_width
    )
}

pub fn get_random_art() -> String {
    let one = "
                                                                    ..%%%%% 
                                                          ...    .%%%%%%\"\" 
                                                       .%%%%%%%/%%%%%\" 
                                     .....           .%%%%%%%%%%%%%%\\ 
                                ..:::::::::::..      :;\"\"  {_}\\%%/%%%%%%.. 
                              .:::::::::::::::::.            {_}/{_}%%%%%%% 
                             :::::::::::::::::::::            \\\\//    \"\"\"\\:: 
                            :::::::::::::::::::::::           \\\\// 
---____-----__---------_____----....-----..--.....~-----_____-\\\\//---_____-
-----___---___----____--__--_.----...----...----..---__--_-___\\\\//--___--__
--____----__-___-----____----_...-----..--..---..___-------__\\\\//_-----____
___----____------____---__--___..--..-..----....___-----___--\\\\//_---__----
---___--___--__---__----___---___..----------._---____-----__\\\\//___---__--
--___---_-___-------______------___--___----___--__------___-\\\\//__--__----
____------____----_________------__                         \\\\//
--_____--__--____                              '            \\\\//  _
__---                 @            ,                  \"          {_}
             .                            '                            
             ";

    let two = "
                                                 *******
                                 ~             *---*******
                                ~             *-----*******
                         ~                   *-------*******
                        __      _   _!__     *-------*******
                   _   /  \\_  _/ \\  |::| ___ **-----********   ~
                 _/ \\_/^    \\/   ^\\/|::|\\|:|  **---*****/^\\_
              /\\/  ^ /  ^    / ^ ___|::|_|:|_/\\_******/  ^  \\
             /  \\  _/ ^ ^   /    |::|--|:|---|  \\__/  ^     ^\\___
           _/_^  \\/  ^    _/ ^   |::|::|:|-::| ^ /_  ^    ^  ^   \\_
          /   \\^ /    /\\ /       |::|--|:|:--|  /  \\        ^      \\
         /     \\/    /  /        |::|::|:|:-:| / ^  \\  ^      ^     \\
   _Q   / _Q  _Q_Q  / _Q    _Q   |::|::|:|:::|/    ^ \\   _Q      ^
  /_\\)   /_\\)/_/\\\\)  /_\\)  /_\\)  |::|::|:|:::|          /_\\)
_O|/O___O|/O_OO|/O__O|/O__O|/O__________________________O|/O__________
//////////////////////////////////////////////////////////////////////
";

    let three = "
            ^^                   @@@@@@@@@
       ^^       ^^            @@@@@@@@@@@@@@@
                            @@@@@@@@@@@@@@@@@@              ^^
                           @@@@@@@@@@@@@@@@@@@@
 ~~~~ ~~ ~~~~~ ~~~~~~~~ ~~ &&&&&&&&&&&&&&&&&&&& ~~~~~~~ ~~~~~~~~~~~ ~~~
 ~         ~~   ~  ~       ~~~~~~~~~~~~~~~~~~~~ ~       ~~     ~~ ~
   ~      ~~      ~~ ~~ ~~  ~~~~~~~~~~~~~ ~~~~  ~     ~~~    ~ ~~~  ~ ~~
   ~  ~~     ~         ~      ~~~~~~  ~~ ~~~       ~~ ~ ~~  ~~ ~
 ~  ~       ~ ~      ~           ~~ ~~~~~~  ~      ~~  ~             ~~
       ~             ~        ~      ~      ~~   ~             ~
       ";

    let four = "
                                                 __
                      ,-_                  (`  ).
                      |-_'-,              (     ).
                      |-_'-'           _(        '`.
             _        |-_'/        .=(`(      .     )
            /;-,_     |-_'        (     (.__.:-`-_.'
           /-.-;,-,___|'          `(       ) )
          /;-;-;-;_;_/|\\_ _ _ _ _   ` __.:'   )
             x_( __`|_P_|`-;-;-;,|        `--'
             |\\ \\    _||   `-;-;-'
             | \\`   -_|.      '-'
             | /   /-_| `\\
             |/   ,'-_|  \\
             /____|'-_|___\\
      _..,____]__|_\\-_'|_[___,.._
     '                          ``'--,..,.      
     ";

    let six = "
                           (   )
                          (    )
                           (    )
                          (    )
                            )  )
                           (  (                  /\\
                            (_)                 /  \\  /\\
                    ________[_]________      /\\/    \\/  \\
           /\\      /\\        ______    \\    /   /\\/\\  /\\/\\
          /  \\    //_\\       \\    /\\    \\  /\\/\\/    \\/    \\
   /\\    / /\\/\\  //___\\       \\__/  \\    \\/
  /  \\  /\\/    \\//_____\\       \\ |[]|     \\
 /\\/\\/\\/       //_______\\       \\|__|      \\
/      \\      /XXXXXXXXXX\\                  \\
        \\    /_I_II  I__I_\\__________________\\
               I_I|  I__I_____[]_|_[]_____I
               I_II  I__I_____[]_|_[]_____I
               I II__I  I     XXXXXXX     I
            ~~~~~\"   \"~~~~~~~~~~~~~~~~~~~~~~~~
    ";

    let seven = "
  ` : | | | |:  ||  :     `  :  |  |+|: | : : :|   .        `              .
      ` : | :|  ||  |:  :    `  |  | :| : | : |:   |  .                    :
         .' ':  ||  |:  |  '       ` || | : | |: : |   .  `           .   :.
                `'  ||  |  ' |   *    ` : | | :| |*|  :   :               :|
        *    *       `  |  : :  |  .      ` ' :| | :| . : :         *   :.||
             .`            | |  |  : .:|       ` | || | : |: |          | ||
      '          .         + `  |  :  .: .         '| | : :| :    .   |:| ||
         .                 .    ` *|  || :       `    | | :| | :      |:| |
 .                .          .        || |.: *          | || : :     :|||
        .            .   . *    .   .  ` |||.  +        + '| |||  .  ||`
     .             *              .     +:`|!             . ||||  :.||`
 +                      .                ..!|*          . | :`||+ |||`
     .                         +      : |||`        .| :| | | |.| ||`     .
       *     +   '               +  :|| |`     :.+. || || | |:`|| `
                            .      .||` .    ..|| | |: '` `| | |`  +
  .       +++                      ||        !|!: `       :| |
              +         .      .    | .      `|||.:      .||    .      .    `
          '                           `|.   .  `:|||   + ||'     `
  __    +      *                         `'       `'|.    `:
\"'  `---\"\"\"----....____,..^---`^``----.,.___          `.    `.  .    ____,.,-
    ___,--'\"\"`---\"'   ^  ^ ^        ^       \"\"\"'---,..___ __,..---\"\"'
--\"'                           ^                         ``--..,__";

    let options = [one, two, three, four, six, seven];
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..options.len());
    let chosen_option = options[random_index];
    chosen_option.to_string()
}
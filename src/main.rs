//made by Zel76 as an entry point to the Rust programming language
//also check out my game made with Godot ---> https://zel76.itch.io/tidy-cat
fn main()
{
    //enter the binary number here
    // TODO: replace them but don't erase them!
    let bin: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]; //here is the TODO
    let place: [u8; 16] = [16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
    let value: [u32; 16] = [32768, 16384, 8192, 4096, 2048, 1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1];
    //the final number variable
    let mut sum: u32 = 0;
    let translate_to_letters: bool = true; //change this if you don't want to translate to letters.
    let mut ascii_letter: char = ' ';


    //translation form bin to dec
    for (_x, &_z) in bin.iter().enumerate()
    {
        if bin[_x] == 1
        {
            sum += value[_x];
            println!("bin {}. is dec {}", place[_x], value[_x]);
        }
    }
    println!("{}", sum);

    //to ascii letters
    if translate_to_letters == true
    {
        if sum == 97
        {
            ascii_letter = 'a';
        }
        else if  sum == 98
        {
            ascii_letter = 'b';
        }
        else if sum == 99
        {
            ascii_letter = 'c';
        }
        else if sum == 100
        {
            ascii_letter = 'd';
        }
        else if sum == 101
        {
            ascii_letter = 'e';
        }
        else if sum == 102
        {
            ascii_letter = 'f';
        }
        else if sum == 103
        {
            ascii_letter = 'g';
        }
        else if sum == 104
        {
            ascii_letter = 'h';
        }
        else if sum == 105
        {
            ascii_letter = 'i';
        }
        else if sum == 106
        {
            ascii_letter = 'j';
        }
        else if sum == 107
        {
            ascii_letter = 'k';
        }
        else if sum == 108
        {
            ascii_letter = 'l';
        }
        else if sum == 109
        {
            ascii_letter = 'm';
        }
        else if sum == 110
        {
            ascii_letter = 'n';
        }
        else if sum == 111
        {
            ascii_letter = 'o';
        }
        else if sum == 112
        {
            ascii_letter = 'p';
        }
        else if sum == 113
        {
            ascii_letter = 'q';
        }
        else if sum == 114
        {
            ascii_letter = 'r';
        }
        else if sum == 115
        {
            ascii_letter = 's';
        }
        else if sum == 116
        {
            ascii_letter = 't';
        }
        else if sum == 117
        {
            ascii_letter = 'u';
        }
        else if sum == 118
        {
            ascii_letter = 'v';
        }
        else if sum == 119
        {
            ascii_letter = 'w';
        }
        else if sum == 120
        {
            ascii_letter = 'x';
        }
        else if sum == 121
        {
            ascii_letter = 'y';
        }
        else if sum == 122
        {
            ascii_letter = 'z';
        }

        println!(" ");
        println!("ASCII letter: {}", ascii_letter);
    }
}
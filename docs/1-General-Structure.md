# General Structure

```Doesnt work rn```

Here you will see the general structure of a shork program. Take, for example, the [hello sea](../shork-examples/hello_sea.sho) example:

```txt
reef examples

get std::io

define example(){
    io::print("Hello Sea 🦈")
}
```

The first line uses a keyword called "reef". You can think of a reef as a package that contains every single source file in a directory. This also means that each directory may only contain one reef, and each reef may only contain one main function. However, a reef can contain multiple directories, so something like that:

```txt
maindir
|-- i_am_in_a_reef.sho
|-- otherdir
    |-- i_am_in_the_same_reef.sho
```

would be perfectly fine.

After the reef stuff, you include libraries with the "get" keyword (explained a bit more in [including libraries](2-Including-Libraries.md)) and then define your functions with the "define" keyword (functions are explained a bit more in [writing functions](3-Writing-Functions.md)). Although you could run the above program with executing ```shork shork-examples/hello_sea.sho --fn example```, your program should contain a main function, which is defined like this:

```txt
define main(){

}
```

Now, you just need to type ```shork the/path/to/my/program/file.sho``` and it'll execute.

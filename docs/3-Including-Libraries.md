# Including libraries

```Doesn't work rn```

As written in [general structure](1-General-Structure.md), libraries are included with the "get" keyword.

For example:

```txt
get std::io
```

Other keywords that you may use in an include are "from" and "as". As you might've guessed, "from" lets you specify a path, and as lets you define another name for the library you included.

For example:

```txt
get sea::squid from ./animals/sea_animals as calamari
```

Inside the animals/sea_animals directory should be a reef called "sea" which contains the file "squid.sho". Functions from that file can then be executed like ```calamari.some_function()``` since the "as" keyword renamed the import.
If no "from" keyword is provided, the shork interpreter will first look into the installed packages in the shork installation directory. If no reef is found there that matches the name, it'll look locally. The first word of the import will be the directory name it'll look (so ```get sea::squid``` would result in the interpreter looking for a "sea" directory with a "sea" reef).

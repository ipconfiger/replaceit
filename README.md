# replaceit
Replace variables in template file for sed haters(方便的替换模版文件中的变量，用于生成配置文件)

70% code write by ChatGPT (本项目70%的代码都是ChatGPT生成的)

But without proper question to ask, it won't be properly build (但是如果不会正确的提问，也得不到正确的程序)

# usage (怎么用)

1. Download from release (从release页面下载对应版本)
   
2. make a soft link (建立一个链接到命令，好到处可以执行)
    ` ln -s /path/to/replaceit /usr/local/bin/replaceit`
   
3. use it (用它)
    ` replaceit template_file_path -v key1=value key2=value`

if there's configure file template:

temp_conf.toml


    [db]
    db_url=${DB_URI}
    db_port=${PORT}


use command (通过命令) `replaceit ./temp_conf.toml -v DB_URI=postgres://alex:123@localhost/testdb PORT=6543`

you can get (就可以得到替换的)


    [db]
    db_url=postgres://alex:123@localhost/testdb
    db_port=6543



if your want save to file, use output redirection （ 如果要保存到文件，使用重定向输出符号）

`replaceit ./temp_conf.toml -v DB_URI=postgres://alex:123@localhost/testdb PORT=6543 > conf.toml`
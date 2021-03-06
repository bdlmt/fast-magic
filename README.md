# fast-magic
Concurrent file search, using libmagic and regex. <br>
<br>
Fast-magic uses all logical CPU cores to concurrently read and scan each file in a directory tree using libmagic.<br>
By default, it will scan all files under the current directory.<br>
A regex pattern is used to return only matching file types. By default, it will match on any file type.<br>

# Usage
<pre>
USAGE:
    fast-magic [FLAGS] [OPTIONS]

FLAGS:
    -h, --help        Prints help information
    -s, --symlinks    Follow symlinks
    -V, --version     Prints version information

OPTIONS:
    -d, --directory <directory>    Directory to walk
    -m, --magic <magic>            Path to magic database file
    -r, --regex <regex>            Regex pattern to match
</pre>

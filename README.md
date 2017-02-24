#### Example

Install gems:

    bundle -j4

Build Rust extension with thermite

    rake thermite:build

Fire up a console:

    ./bin/consle

Get a new empty string from the Rust side:

    2.3.3 :001 > StringTest.new.empty_string
    => ""
    2.3.3 :001 > StringTest.new.empty_string.encoding
    => #<Encoding:ASCII-8BIT>

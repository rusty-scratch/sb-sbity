# mc-sbity

Rust crate with Serde implementation for json part in .sb3 format. (the Scratch 3 project format)

'extensions' and 'text_to_speech_language' has yet to be implemented (I'm tired) but is planned.
Mostly anything else is completely (*probably*) implemented.

Some documentation are taken from the Scratch Wiki.

# Usage
To use this crate you have to unzip the .sb3 file to get its content which contains "project.json" file.
Deserialize "project.json" file with this crate and done!
Do whatever you want with the deserialized data.
Though, this crate were made to be use with my other in development crate. (It's a Scratch project builder 😀)

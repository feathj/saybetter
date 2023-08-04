Saybetter: A simple mac-os "say" like wrapper for the Google text-to-speech engine
-----------------------------------------------------------------------------------
Why? Because there was a huge incongruence between the extremely smart text responses
coming from llama2 and other LLMs, and my computer "say"ing those responses. It didn't
matter which voice I used, I couldn't get over the crappy TTS.

After experimenting with a few different open source models to no avail, I found google's
text-to-speech synthesis to be superior to anything else I have tried. I wasn't able to
find a decent CLI wrapper for the service, so I made one.

Installation
============
This is a rust app, so clone and `cargo build --release`.

To run, you will need to have a google cloud account, with a a project that has "Cloud Text-to-Speech API" enabled.

1. Login to GCM
2. Create new project, take note of the `project name` for use later
3. Search for the "Cloud Text-to-Speech API" and enable it for this project

You will also need to install the [gcloud cli](https://cloud.google.com/sdk/docs/install) to generate tokens to be passed-in.

Running
=======
```sh
./target/release/saybetter \
  --token $(gcloud auth print-access-token) \
  --project "saybetter" \
  --message "This is how you will run it"
```

```sh
echo "Note that it also works with stdin" | ./target/release/saybetter --token $(gcloud auth print-access-token) --project "saybetter"
```

See full list of supported voices / languages [here](https://cloud.google.com/text-to-speech/docs/voices)
# S.A.M.
Smart Artificial Mind
WIP. Dont use this software yet.
Licensed under GPL version 3.

TTS API:
https://tts.opensam.foundation/api/tts?text=hello%20world&speaker_id=&style_wav=
https://tts.alpha.opensam.foundation/api/tts?text=hello%20world&speaker_id=&style_wav=


Features:
- Touch Friendly User Interface
- Built with rust-lang
- Open Source (GPLv3)

Lifx:
  - Works online and offline (with offline lifx server package)
  - Ability to set the light color/kelvin/power from touch panels

Media Center Features:
  - Games
  - App Support (Netflix, Youtube, Spotify, etc.)
  - Game Controller Support

TODO:
- Add clock_widget_display_format setting for the clock widget
- Intergrate Whisper as a primary STT/TTS engine https://github.com/ggerganov/whisper.cpp
- Use whisper for realtime STT/TTS using wasm2js
- Keep exsting TTS/STT methods to be used as a backup
- Add metadata to file storage api

0.0.3:
- Fix file browser (dropbox, deleting files, moving files, etc.)
- Add support for visual RSTP observations
- amd64 support for ffmpeg and whisper packages
- Copy fonts file to sam directory during setup
- Finish package installer (search, install, uninstall)
- Fix settings to actually do something
- Redesign humans page with avatar support
- Fix tracker for heard_count
- Add ability to correct observations in the observation deck
- Review build sprec code
- Redesign notifications to be instant when initiadted from the client side
- Link web session microphone to new sound pipeline s1,s2,s3
- Associate observations with things and/or web sessions
- Redesign locations UIX
- Build calendar/clock widget


AI Features to Expolore:
- Image Super Resoloution
- Minst handwriting
- Speech recognition (DONE)
- Speaker recognition (DONE)
- Deep Vision
- GAN art generation
- Ability to generate reports on any topic
- Ability to sumerize news stories and prioritize feed based on user preferences


Socket API:
- Notifications
- Ability to launch web apps/files on individual devices (web->socket sesssions)

Weather API:
- Uses in house server package "Jupiter" to generate weather reports for your zip code

Console:
- whoami: returns current human users name
- whereami: returns current human users location

News API:
- Copy RSS feed comsuption code from BOT
- Use rust-bert to generate summaries for articles

Stonks API:
- Ability to track blockchain prices
- Ability to track stock prices
- Ability to track multiple wallet realtime values using public keys
- Ability to set high/low notifications

Notifications API:
- Store in SQL
- User Specific
- Can be location specific, but not necessarily
- Ability to send notifications via email or SMS after X minutes of being unread

Storage API:
- Ability to store files in SQL, Cloud Storage, etc. (WIP)

Media Center:
- Controller Support (DONE)
- Music (Depends on new storage pipeline)
- Movies (Depends on new storage pipeline)
- Make games packages that can be installed into the storage pipeline

Settings:
- Ability to set audio recognition noise threshold (WIP)
- Ability to set custom STT server (WIP)

Speech pipeline redesign:
- Recode web recorder to store wav files in the stream pipeline instead of running stt in the browser
- Stitch continuous speech wav file streams and trim whitenoise (DONEish)
- Perform STT using API from settings
- Use sprec to identify the speaker
- Store speech as an observation 
- Split pipeline stream by thing_oid:xxx AND web_session:xxx

Remote Device Management API:
- Ability to execute remote commands over an ssh tunnel connection
- Installs remote utility which gathers system information
- Very useful for keeping multiple touch screen panels, servers updated

Network Security API:
- Scans local network for security vulnerabilities and exposed ports
- Allows sam to make recomendations for incresing network security

#### To Override DNS:
{
  "TaskTemplate": {
      "DNSConfig": {
        "Nameservers": [
          "172.16.0.15"
        ]
      }
    }
}
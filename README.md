# Typst-Renderer-Discord-Bot
Discord bot that can render Typst, an open source rust typesetting project. Useful for sharing maths in discord servers.
Written in Rust and thus (hopefully) fast.

REQUIREMENTS:
Windows 11
Typst: https://github.com/typst/typst/releases
Discord bot with appropriate settings.
- Read and Create Messages
- All three intents
- Store the token in DISCORD_BOT_TOKEN environment variable.

COMMANDS:
!render -> Renders a Typst code block (do not specify language). This is done by:
!render
```
Typst Code Here
```
!rm / !rendermath -> Renders a typst equation from a code block. This is done by:
!rm/!rendermath
```
Typst Equation Here
```

!qm / !quickmath -> Renders a typst equation from the text after. This is done by:
!qm Typst Equation Here

In text equation parsing. This is done by:
Some sentence $Typst Equation here$

!affirmation -> Reads a motivational quote from affirmations.txt
!fact -> Reads a fun fact from facts.txt
!anatomy -> Reads a random muscle / organ / bone from anatomy.txt

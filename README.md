# Embed Creator

Discord app that let's you send embeds anywhere

This is quite a minimalistic Discord application; it only connects to the
gateway briefly when it first starts up to register its `/embed` command, which
is the only command. After that, it just keeps up an interactions endpoint for
Discord to post to whenever there's an interaction.

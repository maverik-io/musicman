<div align=center>
<h1>Music Manager</h1>
  <p>Simple cli for playing music in the <code>~/Music</code> folder</p>
</div>

## What is musicman
Recursively scans the music directory for files (all files, so please only have music in there :P) and allows for playback of found songs. 
Has a queue implemented. Queues can be saved as playlists for quick access.

## Commands
running musicman presents you with a prompt. 

```
$ musicman
musicman❯
```

## The available commands
### add
adds matches to you queue. 

`Syntax: add <search_term>`

>will prompt you to choose if multiple matches are found!

### show | ls
show the current queue.

`Syntax: show`<br>
`Syntax: ls`

### p | play | pause
pause/resume playback

`Syntax: p`<br>
`Syntax: play`<br>
`Syntax: pause`

### clear
clears queue

`Syntax: clear`

### next 
skip to the next song in queue

`Syntax: next`

### prev
skip to the previous song in queue

`Syntax: prev`

### replay
replay the current song.

`Syntax: replay`

### playlist 
playlist creation and playing
#### new
saves the current queue as a playlist
>playlist names are case sensitive (except on MacOS)

<br>

`Syntax: playlist new <name`
#### show | ls
shows all the available playlists<br>
`Syntax: playlist show`<br>
`Syntax: playlist ls`
#### load
load the specified playlist as the queue <br>
`Syntax: playlist new <name`
>replaces the queue!

### exit
quit the player

`Syntax: exit`

-----









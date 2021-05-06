ffmpeg
-y
-loglevel error
-i <input>
-vf scale=200:-1
-ss 00:05
-frames:v 1
<output>
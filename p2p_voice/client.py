import pyaudio
import wave

import socket

# setup audio

CHUNK = 1024
FORMAT = pyaudio.paInt16
CHANNELS = 2
RATE = 44100
RECORD_SECONDS = 1
WAVE_OUTPUT_FILENAME = "output.wav"

# setup UDP stream in
UDP_IP = "127.0.0.1"
UDP_PORT = 5005

# sock_in = socket.socket(socket.AF_INET, # Internet
#                      socket.SOCK_DGRAM) # UDP
# sock_in.bind((UDP_IP, UDP_PORT))

# setup UDP stream out

MESSAGE = b"Hello, World!"

print("UDP target IP:", UDP_IP)
print("UDP target port:", UDP_PORT)
print("message:", MESSAGE)

# sock_out = socket.socket(socket.AF_INET, # Internet
#                      socket.SOCK_DGRAM) # UDP


# input audio
p = pyaudio.PyAudio()

stream_in = p.open(format=FORMAT,
                channels=CHANNELS,
                rate=RATE,
                input=True,
                frames_per_buffer=CHUNK)

# output audio
# stream_out = p.open(
#     format = FORMAT,
#     channels = CHANNELS,
#     rate = RATE,
#     output = True,
#     frames_per_buffer=CHUNK
# )

print("* recording")

frames = []

silence = chr(1)*CHUNK*CHANNELS*2 

for i in range(0, int(RATE / CHUNK * RECORD_SECONDS)):
    data = stream_in.read(CHUNK)

    if data == b'':
        data = silence
    else:
        frames.append(data)

    # audio
    # sock_out.sendto(data, (UDP_IP, UDP_PORT))
    # data_in, addr = sock_in.recvfrom(CHUNK) # buffer size is 1024 bytes
    # stream_out.write(silence)
    # print("received message:", data_in)

print(type(data))
print(len(data))

print("* done recording")


stream_in.stop_stream()
stream_in.close()



wf = wave.open(WAVE_OUTPUT_FILENAME, 'wb')
wf.setnchannels(CHANNELS)
wf.setsampwidth(p.get_sample_size(FORMAT))
wf.setframerate(RATE)
wf.writeframes(b''.join(frames))
wf.close()

# stream_out.write(b''.join(frames))

# wf = wave.open(WAVE_OUTPUT_FILENAME, 'rb')
# data = wf.readframes(CHUNK)
# while data != '':
#     stream_out.write(data)
#     data = wf.readframes(CHUNK)


# stream_out.stop_stream()
# stream_out.close()
# p.terminate()
# sock_out.close()
# sock_in.close()


wf = wave.open(WAVE_OUTPUT_FILENAME, 'rb')

# create an audio object
p = pyaudio.PyAudio()

# open stream based on the wave object which has been input.
stream = p.open(format =
                p.get_format_from_width(wf.getsampwidth()),
                channels = wf.getnchannels(),
                rate = wf.getframerate(),
                output = True)

# read data (based on the chunk size)
data = wf.readframes(CHUNK)

print('data: ', data)

print('silence len:', len(silence))
import random
noise = ''

for i in range(4096):
    if random.randint(0,2) == 1:
        noise += '1'
    else:
        noise += '0'

# play stream (looping from beginning of file to the end)
while data != silence:
    # writing to the stream is what *actually* plays the sound.
    stream.write(data)
    data = wf.readframes(CHUNK)

# cleanup stuff.
stream.close()    
p.terminate()
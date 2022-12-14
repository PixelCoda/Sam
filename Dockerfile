FROM debian:latest
MAINTAINER caleb <calebsmithwoolrich@gmail.com>

RUN apt update
RUN apt upgrade -y
RUN apt install libx264-dev libavcodec-extra58 python3 pip wget libboost-dev libopencv-dev python3-opencv ffmpeg iputils-ping libasound2-dev libpulse-dev libvorbisidec-dev libvorbis-dev libopus-dev libflac-dev libsoxr-dev alsa-utils libavahi-client-dev avahi-daemon libexpat1-dev -y
RUN pip3 install rivescript pexpect

RUN mkdir -p /app && cd /app \
&& wget -O /app/sam.tar.xz https://osf.opensam.foundation/api/package/download/armv7/sam.tar.xz?oid=lGJUmlu4Bs0Hscp \
&& tar -xf /app/sam.tar.xz && chmod +x /app/sam

RUN ls /app/

WORKDIR /app/

# Web Port
EXPOSE 8000

# Web Socker Port
EXPOSE 2794

# Snapcast API Port
EXPOSE 1780
EXPOSE 1705
EXPOSE 1704

CMD ["/app/sam"]

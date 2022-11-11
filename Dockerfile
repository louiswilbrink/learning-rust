FROM ubuntu

RUN apt-get update -y && \
    apt update -y && \
    apt-get install curl -y && \
    apt-get install git -y && \
    apt install vim -y

ADD ./.vimrc /root

RUN git clone https://github.com/VundleVim/Vundle.vim.git ~/.vim/bundle/Vundle.vim

RUN vim +BundleInstall +qall

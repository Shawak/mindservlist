FROM keymetrics/pm2:12-alpine

# set work dir
ENV DIR /app
RUN mkdir -p ${DIR}
WORKDIR ${DIR}

# copy settings
COPY package.json ./
COPY pm2.json ./

# install packages
RUN npm install

CMD [ "pm2-runtime", "start", "pm2.json" ]
# examples of running the docker container with environment variables
#
# DATABASE_URL=postgres://user:pass@host.docker.internal/database
# INVITE_REQUIRED=false
# ENVIRONMENT=development
# BCRYPT_COST=8
#
# docker run -e DATABASE_URL=$DATABASE_URL \
#            -e INVITE_REQUIRED=$INVITE_REQUIRED \
#            -e ENVIRONMENT=$ENVIRONMENT \
#            -e BCRYPT_COST=$BCRYPT_COST \
#            -p 3137:3137 \
#            diary.computer:latest
#
# or run using an .env file for environment variables
docker run --env-file backend/.env.docker \
           -p 3137:3137 \
           diary.computer:latest

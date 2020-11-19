db.auth('faxtopRoot', 'faxtopRoot');

var myDb = db.getSiblingDB('faxtop');
myDb.createCollection('deleteMe');

myDb.createUser({
  user: 'faxtopRoot',
  pwd: 'faxtopRoot',
  roles: [
    {
      role: 'readWrite',
      db: 'faxtop',
    },
  ],
});

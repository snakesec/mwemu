rm -rf /opt/ANDRAX/mwemu

cargo build --release

if [ $? -eq 0 ]
then
  # Result is OK! Just continue...
  echo "Cargo build... PASS!"
else
  # houston we have a problem
  exit 1
fi

strip target/release/mwemu

if [ $? -eq 0 ]
then
  # Result is OK! Just continue...
  echo "Strip binary... PASS!"
else
  # houston we have a problem
  exit 1
fi

mkdir /opt/ANDRAX/mwemu

cp -Rf script_examples maps32 maps64 /opt/ANDRAX/mwemu/

cp -Rf target/release/mwemu /opt/ANDRAX/mwemu/

if [ $? -eq 0 ]
then
  # Result is OK! Just continue...
  echo "Strip binary... PASS!"
else
  # houston we have a problem
  exit 1
fi

cp -Rf andraxbin/* /opt/ANDRAX/bin

chown -R andrax:andrax /opt/ANDRAX/

# Contributing to bootkit

## How to develop

On top of `cargo`, you'll need `sqlite3` installed to compile the project.

After you have sqlite3 installed, run:
```
./scripts/setup_local_db.sh
```

After you've set `DATABASE_URL` env variable as instructed, you can compile this with `cargo build`.
If you get `sqlx` related error, it means you didn't set the `DATABASE_URL` env variable correctly.

### Running on a VM

While this *mostly* supports development locally, it's recommended to use a virtual machine (with snapshots) to properly test it.

To add all the required systemd related files to your VM, run these commands:
```sh
VM_IP=root@10.0.0.1
scp dbus/org.opensuse.bootkit.conf $VM_IP:/usr/share/dbus-1/system.d/org.opensuse.bootkit.conf
scp dbus/bootkitd.service $VM_IP:/usr/lib/systemd/system/
scp dbus/org.opensuse.bootkit.service $VM_IP:/usr/share/dbus-1/system-services/
ssh $VM_IP mkdir -p /var/lib/bootkit
ssh $VM_IP touch /var/lib/bootkit/bootkit.db
```

After you've done that, you can run these commands to run your local build on the target VM:
```sh
VM_IP=root@10.0.0.1
cargo build --release
ssh $VM_IP killall bootkitd || true
scp target/release/bootkit $VM_IP:/sbin/bootkitd
ssh $VM_IP bootkitd -l debug --pretty
```

(It's recommended to add these commands to a script to make running after you make modifications.)

After this, you can confirm that it's running with this command (as root):
```sh
busctl introspect org.opensuse.bootkit /org/opensuse/bootkit
```

### Running locally (not recommended)

**Running as root locally might accidentally break your bootloader configs**

If you're willing to be careful, this can be run locally with:
```sh
./scripts/run_dev.sh
```

And to check that it's running:
```sh
busctl --user introspect org.opensuse.bootkit /org/opensuse/bootkit
```

This uses example data from `test_data` instead of your bootloader configs.
Generally things that fetch data work, but things that modify bootloader configs don't.

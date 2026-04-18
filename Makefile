default:
	$(MAKE) build
	sudo $(MAKE) install

build:
	cargo build --release

install:
	install -Dm755 ./target/release/filemanager1 $(DESTDIR)/usr/bin/
	install -Dm644 ./assets/org.freedesktop.FileManager1.service $(DESTDIR)/usr/share/dbus-1/services/
	install -Dm644 ./assets/filemanager1.service ${DESTDIR}/usr/lib/systemd/user/ 

clean:
	rm $(DESTDIR)/usr/bin/filemanager1
	rm $(DESTDIR)/usr/share/dbus-1/services/org.freedesktop.FileManager1.service
	rm ${DESTDIR}/usr/lib/systemd/user/filemanager1.service

reload:
	systemctl --user daemon-reload
	systemctl --user restart filemanager1.service 

.PHONY: build install clean default reload

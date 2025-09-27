.PHONY: sync


sync:
	rsync -avh --delete --progress --exclude 'target' . matt@gleichpi.local:~/microdotphat/
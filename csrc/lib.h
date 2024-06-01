#include <linux/sed-opal.h>

/*
 * as per NVME base specification 2.0b
 */
struct nvme_id
{
	__u16 pci_vid;
	__u16 pci_ssvid;
	unsigned char serial_number[20];
	unsigned char model_number[40];
	unsigned char firmware_revision[8];
};

int get_status(int fd, struct opal_status *opal_status);
int identify(int fd, struct nvme_id *out);


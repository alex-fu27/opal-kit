#include "lib.h"
#include <sys/ioctl.h>
#include <string.h>
#include <linux/sed-opal.h>
#include <linux/nvme_ioctl.h>

#define NVME_IDENTIFY 0x06
#define NVME_IDENTIFY_CONTROLLER 0x01

/*
 * the identify command actually is 256 bytes, but we care only about the first couple of fields
 */
struct nvme_id_buffer
{
	struct nvme_id relevant;
	__u8 filler[4096 - sizeof(struct nvme_id)];
};


int get_status(int fd, struct opal_status *opal_status)
{
	return ioctl(fd, IOC_OPAL_GET_STATUS, opal_status);
}

int identify(int fd, struct nvme_id *out)
{
	struct nvme_id_buffer buf;
	struct nvme_passthru_cmd cmd;

	memset(&cmd, 0, sizeof(cmd));
	cmd.opcode = NVME_IDENTIFY;
	cmd.addr = (unsigned long) &buf;
	cmd.data_len = sizeof(buf);
	cmd.cdw10 = NVME_IDENTIFY_CONTROLLER;
	// others left as 0

	int err = ioctl(fd, NVME_IOCTL_ADMIN_CMD, &cmd);
	if (err)
		return err;

	memcpy(out, &buf.relevant, sizeof(*out));
	return 0;
}


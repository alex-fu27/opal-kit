#include "lib.h"
#include <fcntl.h>
#include <linux/sed-opal.h>

int get_status(int fd, struct opal_status &opal_status)
{
	return ioctl(fd, IOC_OPAL_GET_STATUS, opal_status);
}


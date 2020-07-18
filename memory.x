/* Memory Layout of the STM32F303VCT6 */
/* NOTE 1 K = 1 KiBi = 1024 bytes */
MEMORY
{
    FLASH : ORIGIN = 0x08000000, LENGTH = 256K
    RAM : ORIGIN = 0x20000000, LENGTH = 40K
}
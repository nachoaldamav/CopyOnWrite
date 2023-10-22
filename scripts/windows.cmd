:: Prepare Disk
ECHO ::group::Prepare Disk
SET driveLetter=Z:
SET volumeLabel=ReFSVolume
SET diskNumber=1
:: Initialize Disk
DISKPART /s <(
ECHO SELECT DISK %diskNumber%
ECHO CLEAN
ECHO CONVERT GPT
ECHO CREATE PARTITION PRIMARY
ECHO FORMAT FS=REFS LABEL="%volumeLabel%" QUICK
ECHO ASSIGN LETTER=%driveLetter%
ECHO EXIT
)
ECHO ::endgroup::

:: Create "code" directory in the ReFS volume
ECHO ::group::Create code directory
MD %driveLetter%\code
ECHO ::endgroup::

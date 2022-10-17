This repo is currently used a learning platform
about using rust to control an arduino nano 33 iot. 

There already exists good libraries out there of which
I'm trying to learn from and here is such place for
the learning. 


# Running tests

Create a test-runner.bat file (if on windows), this will be called
when running cargo test from within the self-tests directory

these test will be ran on the arduino-nano directly, the led will blink if a test fails. 


Using a PI you can toggle the reset pin twice by pulling the pin low two times. 

This will let all the tests be ran by the runner script which you can send a restart to bootloader
command via a PI's gpio pins. This makes it so that running tests without toggling the bootloader manually
is possible. 

to wire up the pi, set a gpio pin that has 3.3v on it and have it toggle low to cause a reset to occur. 

the wiring I used was


pi 23 gpio -> nano reset pin. 
pi ground -> nano ground pin.

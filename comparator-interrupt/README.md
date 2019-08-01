# Comparator interrupt

This example sets up the analog comparator module to compare an input on its negative terminal
to an internal reference voltage of 1.1V on its positive terminal. The device will cause an interrupt
when any edge is detected on the comparator output. The ISR will toggle the user LEDs to match the
current output of the comparator.

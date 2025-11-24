# Code for Matura Project by Jan Wilhelm

## Improvements for live_feed
- [x] Dont resize the image
- [ ] Focus on computation, then gui
- [x] Seperate thread for sending commands
- [ ] Ideas:
    * Search the ball in the original image and then only undistort the part around the ball and then search it again
      * Problems:
        * The ball isnt round anymore
        * Could be slower because of more commputations
    * Try to only undistort the ball coordinates
- [ ] Smooth out the movements of the motor
  * Smaller microsteps
  * Smoother acceleration
  * Maybe use the AccelStepper library
- [ ] Maybe rewrite everything in python
- [ ] switch to vimba library

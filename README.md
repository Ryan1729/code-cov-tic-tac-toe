# Code-Cov Tic-Tac-Toe

This Repo was an experiment to see how effective, (or in effective) keeping 100% test coverage was at influencing development in a positive direction. I was aware that conventional wisdom recommended against 100% code coverage, but I wanted to see for myself.

The results were not very good. It turned out that with a no dependencies application, in a language with hygenic macros available, it was relatively easy to maintain coverage 100% line and function coverage. So maintaining it had little to no effect on development. I found myself writing other tests that were completely not motivated by the coverage and instead by the problem at hand.

Since the tooling at the time made maintaining branch coverage more difficult that it would otherwise be, I did not maintain that for very long. Still, just the raw numbers of branches suggest that maintaining that would indeed be impractical in almost all cases, even if to apparently impossible to take branches around pattern matches were filtered out.

The actual project technically works but is not a good tic-tac-toe AI, (it's pretty easy to win where it should be able to consistently tie). Some tests pointing out specific issues are commented out. Fixing the issues is left "as an exercise the the reader".

# OneWeekeendRayTracer
Implementation of https://raytracing.github.io/books/RayTracingInOneWeekend.html in rust


# Multithreading


Main thread loops over each thread listing for get work.

If a channels requests for. Send work to them or send stop if all work i done. Increment next work count.
If a channel does not have a message try next. loop over all threads until all work has been done and all threads has revieced a done message

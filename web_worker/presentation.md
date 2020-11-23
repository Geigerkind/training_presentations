---
presentation:
  theme: league.css
  margin: 0.1
  enableSpeakerNotes: true
---

<!-- slide -->
<aside class="notes">
</aside>

# Web worker

<!-- slide -->
<aside class="notes">
</aside>

## Web worker ?
Run a script operation in the background thread such that the main thread is not blocked.

<!-- slide -->
<aside class="notes">
1. Heavy lifting, your goal is to provide a lag free experience to the user. So number crunching that may stress the thread can be done in the background.

2. You can load the whole data set once and request filter on it. During filtering the UI will stay smooth until it receives partial or the whole updated subset. With this, the heavy work of doing expensive filter on such big data sets is avoided in the main thread.

3. The main purpose is background syncronization and background tasks. There is a whole topic around it called ServiceWorker, which provides easy ways to implement background services, such as push notifications and easy ways to extend your application to provide an offline experience to your users.
</aside>

## Applications
* Heavy lifting, e.g. number crunching
* Filtering
* Offline App
* Polling

<!-- slide -->
<aside class="notes">
</aside>

## Types
* DedicatedWorker 
* SharedWorker 
* ServiceWorker 

<!-- slide -->
<aside class="notes">
These restrictions apply to all worker.

1. You have no DOM access. Your DOM is the UI, as in all GUI based applications, only the GUI thread, i.e. the main thread can access it.

2. No shared memory. Neither between main and spawned thread, nor between multiple spawned threads. Communication solely via messaging.

3. Besides the DOM, local storage API's like localStorage are not accessible for example. In general, anything that is not thread safe. For a full list, the documentation should be consulted.
</aside>

## General restrictions
* No DOM access
* No shared memory
* Certain window properties inaccessible

<!-- slide -->
<aside class="notes">
As the name says, it creates a dedicated worker for this particular page. Another thread if you will so,
just a little restricted.

Example: https://mdn.github.io/simple-web-worker/

You can see it here (in firefox).
Useful: about:debugging#/runtime/this-firefox

Or in the memory tab in chrome.
</aside>

## DedicatedWorker
* Only accessible by the script that called it
* Only lives for the life cycle of the calling script
* Supported by all browser

<!-- slide -->
<aside class="notes">
In contrast to dedicated worker, shared worker live for the duration of all calling scripts.

Application:
Useful to use a shared state in the worker on multiple sites, e.g. certain data sets and different pages are just different filters.

Unfortunately, there is no support for mobile browser (except Firefox). But this limitation can be overcome by single page applications.

Useful: about:debugging#/runtime/this-firefox
Example: https://mdn.github.io/simple-shared-worker/
</aside>

## SharedWorker
* Accessible by **multiple** scripts
* Not supported by most mobile browser

<!-- slide -->
<aside class="notes">
Essentially a proxy between server and web app. It can intercept network requests and
take the appropriate action based wheter network is available. 

Application:
* Create an offline experience
* Push notifications
* Pre-fetching (performance)
* Other background services
* Centralized updates to expensive-to-calculate data

Useful tools are workbox: https://developers.google.com/web/tools/workbox/
Example: https://github.com/Geigerkind/LegacyPlayersV3/blob/master/Webclient/src/serviceworker.js
</aside>

## ServiceWorker
* Essentially a proxy between server and web app
* Run outside the web app life cycle
* No access to synchronous API
* HTTPS only
* Supported by all browser

<!-- slide -->
<aside class="notes">
Communication solely via messaging.

This is an example for a dedicated worker.
</aside>

## Communication
```javascript
// main.js
var myWorker = new Worker('worker.js');
numberInput.onchange = function() {
  myWorker.postMessage([numberInput.value, numberInput.value]);
  console.log('Message posted to worker');
}
```

```javascript
// worker.js
onmessage = function(e) {
  console.log('Message received from main script');
  var workerResult = 'Result: ' + (e.data[0] * e.data[1]);
  console.log('Posting message back to main script');
  postMessage(workerResult);
}
```


<!-- slide -->
<aside class="notes">
Angular properly supports dedicated worker since Angular 8.

Documentation: https://angular.io/guide/web-worker

Nett to know:
Angular creates single page applications, so for components, services that live long the life cycle has to be manually handled if necessary.
This can also be used to our advantage, since we can abuse the DedicatedWorker as a SharedWorker.

Limitations:
Dependency injection is not manged for WebWorker. This has to be done manually.
</aside>

## Angular
```typescript
// src/app/app.component.ts
if (typeof Worker !== 'undefined') {
  const worker = new Worker('./app.worker', { type: 'module' });
  worker.onmessage = ({ data }) => {
    console.log(`page got message: ${data}`);
  };
  worker.postMessage('hello');
}

```

```typescript
// src/app/app.worker.ts
addEventListener('message', ({ data }) => {
  const response = `worker response to ${data}`;
  postMessage(response);
});
```

<!-- slide -->
<aside class="notes">
Working with web workers is a little tedious because we have to use the bare metal messaging API so rigerously.

But luckily, there is ComLink. Its an remote procedure call implementation: Values from one thread can be used within the other thread (and vice versa) just like local values.

It abstracts away from postMessage and provides a developer friendly API.

Typescript bindings exist.
</aside>

## Comlink to the rescue I
```javascript
// main.js
async function init() {
  const obj = Comlink.wrap(new Worker("worker.js"));
  alert(`Counter: ${await obj.counter}`);
  await obj.inc();
  alert(`Counter: ${await obj.counter}`);
}
init();
```

```javascript
// worker.js
const obj = {
  counter: 0,
  inc() {
    this.counter++;
  },
};
Comlink.expose(obj);
```

<!-- slide -->
<aside class="notes">
Here we execute an expensive callback in the worker.
</aside>

## Comlink to the rescue II
```javascript
// main.js
function callback(value) {
  alert(`Result: ${value}`);
}
async function init() {
  const remoteFunction = Comlink.wrap(new Worker("worker.js"));
  await remoteFunction(Comlink.proxy(callback));
}
init();
```
```javascript
// worker.js
async function remoteFunction(cb) {
  await cb("A string from a worker");
}
Comlink.expose(remoteFunction);
```

<!-- slide -->
<aside class="notes">
Web worker are strong tool. But not used wisely, its rather useless and makes the performance even worse.

So where does it make sense to use web worker?
Messaging is expensive, especially with rich type annotations. Each java object is serialized and deserialized. If the gain does not outweigh that, it is not worth it.

There is no support for Internet Explorer 9 and below, so if you need to support it, I guess you have other things to worry about any way.

It will make your application significantly more complex. So if it is not needed, you should not use it.
If you know that you will be doing a lot of heavy lifting in the Frontend, then it may be worth it, but most
devices can handle rather much work on one thread without imposing too much stress.

</aside>

## Bad use cases
* Messaging overhead > Gain
* Internet Explorer <=9 :'D
* There is no problem to the solution

<!-- slide -->
# Questions
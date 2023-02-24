
const base_url = "https://my_api.com"
const params = new Proxy(new URLSearchParams(window.location.search), {
  get: (searchParams, prop) => searchParams.get(prop),
});

const source = params.source;
const code = params.code;

console.log(`verifying source: ${source} code: ${code}`)

fetch(`${base_url}/verify/${source}/${code}`)
    .then((response) => response.json())
    .then((data) => {
        console.log('Success:', data);
    })
    .catch((error) => {
        console.error('Verify Error:', error);
    });

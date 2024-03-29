// scans/+server.ts
import type { RequestHandler } from '@sveltejs/kit';
import axios from 'axios';
import { DATA_DRAKE_AUTH_TOKEN, BACKEND_URI } from '$env/static/private';

export const GET: RequestHandler = async () => {
  try {
    //console.log(DATA_DRAKE_AUTH_TOKEN)
    const response = await axios.get(`${BACKEND_URI}/get_root_paths`, {
      headers: {
        'Content-Type': 'application/json',
        'Authorization': DATA_DRAKE_AUTH_TOKEN,
      },
    });

    return new Response(JSON.stringify(response.data), {
      status: 200,
      headers: {
        'Content-Type': 'application/json',
      },
    });
  } catch (error) {
    console.error('Error fetching data:', error);
    return new Response(JSON.stringify({ error: 'Internal Server Error' }), {
      status: 500,
      headers: {
        'Content-Type': 'application/json',
      },
    });
  }
};

export const POST: RequestHandler = async ({ request }) => {
  try {
    const data = await request.json();
    const directory = data.path;

    //console.log(DATA_DRAKE_AUTH_TOKEN)
    const response = await axios.post(`${BACKEND_URI}/scan_directory`, { path: directory }, {
      headers: {
        'Content-Type': 'application/json',
        'Authorization': DATA_DRAKE_AUTH_TOKEN,
      },
    });

    return new Response(JSON.stringify(response.data), {
      status: 200,
      headers: {
        'Content-Type': 'application/json',
      },
    });
  } catch (error) {
    console.error('Error fetching data:', error);
    return new Response(JSON.stringify({ error: 'Internal Server Error' }), {
      status: 500,
      headers: {
        'Content-Type': 'application/json',
      },
    });
  }

};

export const DELETE: RequestHandler = async ({ request }) => {
  try {
    const data = await request.json();
    //console.log(data)
    const directory = data.path;

    //console.log(DATA_DRAKE_AUTH_TOKEN);
    const response = await axios.delete(`${BACKEND_URI}/delete_scan`, {
      headers: {
        'Content-Type': 'application/json',
        'Authorization': DATA_DRAKE_AUTH_TOKEN,
      },
      data: { path: directory },
    });

    return new Response(JSON.stringify(response.data), {
      status: 200,
      headers: {
        'Content-Type': 'application/json',
      },
    });
  } catch (error) {
    console.error('Error deleting scan:', error);
    return new Response(JSON.stringify({ error: 'Internal Server Error' }), {
      status: 500,
      headers: {
        'Content-Type': 'application/json',
      },
    });
  }
};
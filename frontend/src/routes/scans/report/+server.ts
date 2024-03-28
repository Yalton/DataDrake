// scans/report/+page.server.ts
import type { RequestHandler } from '@sveltejs/kit';
import axios from 'axios';
import { DATA_DRAKE_AUTH_TOKEN } from '$env/static/private';

export const GET: RequestHandler = async ({ url }) => {
  try {
    const path = url.searchParams.get('path');

    console.log(DATA_DRAKE_AUTH_TOKEN);
    const response = await axios.post('http://127.0.0.1:8000/get_scan', { path }, {
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
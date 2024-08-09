// scans/report/+page.server.ts
import type { RequestHandler } from '@sveltejs/kit';
import axios from 'axios';
import { DATA_DRAKE_AUTH_TOKEN, BACKEND_URI} from '$env/static/private';

export const GET: RequestHandler = async ({ url }) => {
  try {
    const path = url.searchParams.get('path');

    //console.log(DATA_DRAKE_AUTH_TOKEN);
    const response = await axios.post(`${BACKEND_URI}/get_scan`, { path }, {
      headers: {
        'Content-Type': 'application/json',
        'Authorization': DATA_DRAKE_AUTH_TOKEN,
      },
      responseType: 'stream',
    });
    // console.log(response.data)
    return new Response(response.data, {
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
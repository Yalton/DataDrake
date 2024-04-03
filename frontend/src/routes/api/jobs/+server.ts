import type { RequestHandler } from '@sveltejs/kit';
import axios from 'axios';
import { DATA_DRAKE_AUTH_TOKEN, BACKEND_URI} from '$env/static/private';
import type { Job } from '$lib/types';

export const GET: RequestHandler = async () => {
    try {
        let jobs: Job[] = [];

        const response = await axios.get(`${BACKEND_URI}/get_job_status/all`, {
            headers: {
                'Content-Type': 'application/json',
                'Authorization': DATA_DRAKE_AUTH_TOKEN,
            },
        });

        jobs = JSON.parse(response.data);
        ////console.log(jobs);
        ////console.log("Array.isArray(jobs)", Array.isArray(jobs));

        jobs.sort((a, b) => {
            return new Date(b.created_at).getTime() - new Date(a.created_at).getTime();
        });

        return new Response(JSON.stringify(jobs), {
            status: 200,
            headers: {
                'Content-Type': 'application/json',
            },
        });
    } catch (error) {
        console.error('Error fetching jobs:', error);
        return new Response(JSON.stringify({ error: 'Internal Server Error' }), {
            status: 500,
            headers: {
                'Content-Type': 'application/json',
            },
        });
    }
};
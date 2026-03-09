import asyncio


#region gather-jobs
async def collect_jobs(inputs: list[str]) -> list[str]:
    async def upper(item: str) -> str:
        await asyncio.sleep(0.01)
        return item.upper()

    return await asyncio.gather(*(upper(item) for item in inputs))
#endregion gather-jobs

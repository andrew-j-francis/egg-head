export const getCurrentDate = () => {
    let today = new Date();
    let dd = String(today.getDate()).padStart(2, '0');
    let mm = String(today.getMonth() + 1).padStart(2, '0');
    let yyyy = today.getFullYear();

    return yyyy + '-' + mm + '-' + dd;
}

export const checkStartDate = (taskStartDate) => {
    console.log(getCurrentDate());
    console.log(taskStartDate);
    return true;
}
const RbacApi = {
  queryPageUrl: "/rbac/v1/app/find/page",
  publishUrl: "/rbac/v1/app/enable/",
  unPublishUrl: "/rbac/v1/app/disable/",
  reDevelophUrl: "/rbac/v1/app/restart/",
  deleteUrl: "/rbac/v1/app/delete/"
}
const onQueryAppPageData = (queryParams) => {
  queryParams = queryParams || {}
  BmbpHttp.post(RbacApi.queryPageUrl, queryParams).then((resp) => {
    if (resp.code == 0) {
      let respData = resp.data;
      AppPageIns.setPagination({ ...AppPageIns.pagination, total: respData.rowTotal });
      AppPageIns.setGridData(respData.data);
    } else {
      arco.Message.error(resp.msg);
    }
  });
}

const onRowEnableBtnClick = (record) => {
  BmbpHttp.post(RbacApi.publishUrl + record.recordId, {}).then((resp) => {
    if (resp.code == 0) {
      arco.Message.info("应用发布成功");
      onQueryAppPageData({});
    } else {
      arco.Message.error(resp.msg);
    }
  });
}

const onRowDisableBtnClick = (record) => {
  BmbpHttp.post(RbacApi.unPublishUrl + record.recordId, {}).then((resp) => {
    if (resp.code == 0) {
      arco.Message.info("应用下线成功");
      onQueryAppPageData({});
    } else {
      arco.Message.error(resp.msg);
    }
  });
}

const onRowReStartBtnClick = (record) => {
  BmbpHttp.post(RbacApi.reDevelophUrl + record.recordId, {}).then((resp) => {
    if (resp.code == 0) {
      arco.Message.info("应用重启开发成功");
      onQueryAppPageData({});
    } else {
      arco.Message.error(resp.msg);
    }
  });
}


const onRowDelBtnClick = (record) => {
  BmbpHttp.post(RbacApi.deleteUrl + record.recordId, {}).then((resp) => {
    if (resp.code == 0) {
      arco.Message.info("应用删除成功");
      onQueryAppPageData({});
    } else {
      arco.Message.error(resp.msg);
    }
  });
}

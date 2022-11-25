<template>
    <el-row>
        <el-select clearable v-model="value" class="m-2" placeholder="选择存储提供商">
            <el-option v-for="item in options" :key="item.value" :label="item.label" :value="item.value" />
        </el-select>
        <el-button type="primary" @click="openEditDialog" style="margin-left: 0.8em;">新增</el-button>

    </el-row>
    <el-table :data="store.list" style="width: 100%;margin-top: 0.5em;">
        <el-table-column prop="name" label="名称" width="180" />
        <el-table-column prop="app_key" label="AppKey" width="400" />
        <el-table-column prop="secret" label="Secret" width="380"  />
        <el-table-column prop="description" label="备注" />
        <el-table-column label="创建时间" width="180">
            <template #default="row">
                <div> {{ datetimeFormat(row.create_time) }}</div>
            </template>
        </el-table-column>
        <el-table-column fixed="right" label="操作" width="110">
            <template #default="row">
                <el-button link type="primary" size="small">
                编辑
                </el-button>
                <el-button link type="danger" size="small">
                    删除
                </el-button>
            </template>
        </el-table-column>
    </el-table>
    <el-dialog v-model="editAccountDialogVisible">
        <el-form :model="editAccountForm" label-width="120px">
            <el-form-item label="名称" required>
                <el-input v-model="editAccountForm.name" placeholder="我的个人账号" />
            </el-form-item>
            <el-form-item label="AppKey" required>
                <el-input v-model="editAccountForm.app_key" />
            </el-form-item>
            <el-form-item label="Secret" required>
                <el-input v-model="editAccountForm.secret" />
            </el-form-item>
            <el-form-item label="Key前缀" required>
                <el-input v-model="addition.prefix" placeholder="{prefix}/" />
            </el-form-item>
            <el-form-item label="Key生成策略">
                <el-select v-model="addition.key_policy" placeholder="选择策略" @change="handleKeyPolicyChange">
                    <el-option v-for="opt in keyPolicyOptions" :key="opt.value" :label="opt.label" :value="opt.value">
                    </el-option>
                </el-select>
                <div class="tips">
                    <el-tooltip :content="keyPolicyTips" raw-content>
                        <!-- <el-button>hover me</el-button> -->
                        <el-icon>
                            <InfoFilled />
                        </el-icon>
                    </el-tooltip>
                </div>
            </el-form-item>
            <el-form-item label="备注">
                <el-input v-model="editAccountForm.description" />
            </el-form-item>
        </el-form>
        <div slot="footer" class="dialog-footer">
            <el-button @click="editAccountDialogVisible = false">取 消</el-button>
            <el-button type="primary" @click="saveAccount">确 定</el-button>
        </div>
    </el-dialog>
</template>

<script lang="ts" setup>
import { onMounted, ref, computed } from "vue";
import { Account } from "../model/account";
import { useAccountStore } from "../stores/account";
import moment from "moment";

const addition = ref({
    prefix: '',
    key_policy: 1
})

const datetimeFormat = computed(() => {
    return (date: string) => {
        return moment(date).format('YYYY-MM-DD HH:mm:ss')
    }
})

const editAccountDialogVisible = ref(false)
const editAccountForm = ref<Account>({} as Account)
function openEditDialog() {
    editAccountDialogVisible.value = true
}
const store = useAccountStore()

const value = ref('')

const options = [
    {
        value: 1,
        label: '七牛云',
    },
    {
        value: 2,
        label: '阿里云',
    },
    {
        value: 3,
        label: '腾讯云',
    },
    {
        value: 4,
        label: 'AWS',
    },
    {
        value: 99,
        label: 'minio S3',
    },
]

const keyPolicyOptions = [
    {
        value: 1,
        label: '前缀+UUID',
    },
    {
        value: 2,
        label: '前缀+日期+UUID',
    },
]

const defaultKeyPolicyTips = '<span>{prefix}/{uuid}.{ext} 例如：images/123e4567-e89b-12d3-a456-426655440000.png</span>'
const keyPolicyTips = ref(defaultKeyPolicyTips)
function handleKeyPolicyChange(val: number) {
    if (val === 1) {
        keyPolicyTips.value = defaultKeyPolicyTips
    } else {
        keyPolicyTips.value = '<span>{prefix}/{date}/{uuid}.{ext} 例如：images/20221111/123e4567-e89b-12d3-a456-426655440000.png</span>'
    }
}

function saveAccount() {
    editAccountForm.value.addition = JSON.stringify(addition.value)
    console.log("editAccountForm", editAccountForm.value)
    //todo check
    store.addAccount(editAccountForm.value).then(() => {
        console.log("success")
        editAccountDialogVisible.value = false
    }).catch((err) => {
        console.log(err)
        editAccountDialogVisible.value = false;
    })

}
onMounted(() => {
    console.log(store)
    store.listAccount().then((res) => {
        console.log(res)
    }).catch(err => {
        console.error(err)
    })
})
</script>
<style lang="scss">
.tips {
    margin-left: 0.5em;
}

.dialog-footer {
    text-align: right;
}
</style>
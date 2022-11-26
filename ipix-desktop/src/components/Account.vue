<template>
    <el-row>
        <el-select clearable v-model="value" class="m-2" placeholder="选择存储提供商" @change="handleSelectedProvider">
            <el-option v-for="item in options" :key="item.value" :label="item.label" :value="item.value" />
        </el-select>
        <el-button type="primary" @click="openEditDialog('add')" style="margin-left: 0.8em;">新增</el-button>

    </el-row>
    <el-table :data="store.filterList" style="width: 100%;margin-top: 0.5em;" row-key="id">
        <el-table-column prop="name" label="名称" width="180" />
        <el-table-column prop="app_key" label="AppKey" width="200" show-overflow-tooltip />
        <el-table-column prop="secret" label="Secret" width="200" show-overflow-tooltip />
        <el-table-column label="提供商" width="200" show-overflow-tooltip>
            <template #default="scope">
                <div>{{ storageProviderName(scope.row.provider) }}</div>
            </template>
        </el-table-column>
        <el-table-column prop="description" label="备注" min-width="200" show-overflow-tooltip />
        <el-table-column label="创建时间" width="180">
            <template #default="scope">
                <div> {{ datetimeFormat(scope.row.create_time) }}</div>
            </template>
        </el-table-column>
        <el-table-column fixed="right" label="操作" width="110">
            <template #default="scope">
                <el-button link type="primary" size="small" @click="openEditDialog('edit', scope.row)">
                    编辑
                </el-button>
                <el-button link type="danger" size="small" @click="handleRemoveAccount(scope.row.id)">
                    删除
                </el-button>
            </template>
        </el-table-column>
    </el-table>
    <el-dialog v-model="editAccountDialogVisible" :title="editDialogTitle" center @close="cancelSaveAccount">
        <el-form :model="editAccountForm" label-width="82px">
            <el-form-item label="名称" required>
                <el-input v-model="editAccountForm.name" placeholder="个人账号" />
            </el-form-item>
            <el-form-item label="AppKey" required>
                <el-input v-model="editAccountForm.app_key" />
            </el-form-item>
            <el-form-item label="Secret" required>
                <el-input v-model="editAccountForm.secret" />
            </el-form-item>
            <el-form-item label="存储提供商">
                <el-select clearable v-model="editAccountForm.provider" class="m-2" placeholder="请选择存储提供商">
                    <el-option v-for="item in options" :key="item.value" :label="item.label" :value="item.value" />
                </el-select>
            </el-form-item>
            <el-form-item label="存储桶" required>
                <el-input v-model="addition.bucket"></el-input>
            </el-form-item>
            <el-form-item label="域名" required>
                <el-input v-model="addition.host"></el-input>
            </el-form-item>
            <!-- <el-form-item label="Key前缀" required>
                <el-input v-model="addition.prefix" placeholder="{prefix}/" />
            </el-form-item>
            <el-form-item label="Key生成策略">
                <el-select v-model="addition.key_policy" placeholder="选择策略" @change="handleKeyPolicyChange">
                    <el-option v-for="opt in keyPolicyOptions" :key="opt.value" :label="opt.label" :value="opt.value">
                    </el-option>
                </el-select>
                <div class="tips">
                    <el-tooltip :content="keyPolicyTips" raw-content>
                        <el-icon>
                            <InfoFilled />
                        </el-icon>
                    </el-tooltip>
                </div>
            </el-form-item> -->
            <el-form-item label="备注">
                <el-input v-model="editAccountForm.description" />
            </el-form-item>
        </el-form>
        <div slot="footer" class="dialog-footer">
            <el-button @click="cancelSaveAccount">取 消</el-button>
            <el-button type="primary" @click="saveAccount">确 定</el-button>
        </div>
    </el-dialog>
</template>

<script lang="ts" setup>
import { onMounted, ref, computed } from "vue";
import { Account } from "@/dto/account";
import { useAccountStore } from "@/stores/account";
import moment from "moment";

// account strore
const store = useAccountStore()

// addition is using for saving addition info,such as host,prefix,key_policy
const addition = ref({
    bucket: '',
    host: '',

    // prefix: '',
    // key_policy: 1
})

// edit account dialog mod, defualt is add
const editDialogMod = ref('add')

// computed for date format
const datetimeFormat = computed(() => {
    return (date: string) => {
        return moment(date).format('YYYY-MM-DD HH:mm:ss')
    }
})

const editDialogTitle = computed(() => {
    return editDialogMod.value === 'add' ? '新增存储账号' : '编辑存储账号'
})
// handle provider change
function handleSelectedProvider() {
    store.setSelectProvider(parseInt(value.value))
}

// edit account dialog visible true/false
const editAccountDialogVisible = ref(false)
const editAccountForm = ref<Account>({} as Account)
// open edit/add account dialog
function openEditDialog(mod: string, account?: Account) {
    if (mod === 'add') {
        editDialogMod.value = 'add'
        if (!editAccountForm.value) {
            editAccountForm.value = {} as Account
        }
        editAccountDialogVisible.value = true

    } else if (mod === 'edit') {
        editDialogMod.value = 'edit';
        editAccountForm.value = Object.assign({}, account) as Account
        addition.value = JSON.parse(account?.addition || '{}')
        editAccountDialogVisible.value = true
    }
}


const value = ref('')

const options = [
    {
        value: 1,
        label: '七牛云',
    },
    {
        value: 2,
        label: '阿里云(尚未支持)',
    },
    {
        value: 3,
        label: '腾讯云(尚未支持)',
    },
    {
        value: 4,
        label: 'AWS(尚未支持)',
    },
    {
        value: 99,
        label: 'minio S3',
    },
]

const storageProviderName = computed(() => {
    return (id: number) => {
        const provider = options.filter(item => item.value === id)
        if (provider.length > 0) {
            return provider[0].label
        }
        return "未知"
    }
})

// const keyPolicyOptions = [
//     {
//         value: 1,
//         label: '前缀+UUID',
//     },
//     {
//         value: 2,
//         label: '前缀+日期+UUID',
//     },
// ]
function handleRemoveAccount(id: number) {
    store.removeAccount(id).then(() => {
    }).catch((err) => {
        console.error(err)
    })
}
// move to repo component
// const defaultKeyPolicyTips = '<span>{prefix}/{uuid}.{ext} 例如：images/123e4567-e89b-12d3-a456-426655440000.png</span>'
// const keyPolicyTips = ref(defaultKeyPolicyTips)
// function handleKeyPolicyChange(val: number) {
//     if (val === 1) {
//         keyPolicyTips.value = defaultKeyPolicyTips
//     } else {
//         keyPolicyTips.value = '<span>{prefix}/{date}/{uuid}.{ext} 例如：images/20221111/123e4567-e89b-12d3-a456-426655440000.png</span>'
//     }
// }

function saveAccount() {
    // addition is a JSON string
    editAccountForm.value.addition = JSON.stringify(addition.value)
    //check dialog is add or edit
    if (editDialogMod.value === 'add') {
        store.addAccount(editAccountForm.value).then(() => {
            console.log("success")
        }).catch((err) => {
            console.error(err)
        }).finally(() => {
            editAccountDialogVisible.value = false;
        })
    } else if (editDialogMod.value === 'edit') {
        store.updateAccount(editAccountForm.value).then(() => {
            console.log("update success")
        }).catch((err) => {
            console.error(err)
        }).finally(() => {
            editAccountDialogVisible.value = false;
        })
    }

}
function cancelSaveAccount() {
    // if is add, return
    if (editDialogMod.value === 'add') {
        editAccountDialogVisible.value = false
        return
    }

    if (editDialogMod.value === 'edit') {
        //set value to empty
        editAccountForm.value = {} as Account
        addition.value = {
            bucket: '',
            host: '',
        }
    }
    editAccountDialogVisible.value = false
}
onMounted(() => {
    //fetch account list from backend
    store.listAccount().then(() => {
        console.log("fetch list success")
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
    text-align: center;
}
</style>
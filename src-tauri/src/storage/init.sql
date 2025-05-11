-- 数据源表
CREATE TABLE IF NOT EXISTS data_source (
    id TEXT PRIMARY KEY,         
    name TEXT NOT NULL,
    db_type TEXT NOT NULL, 
    host TEXT,
    port INTEGER,
    username TEXT,
    password TEXT,
    database TEXT NOT NULL,    
    extra_params TEXT           
);

-- 代码规范表
CREATE TABLE IF NOT EXISTS code_sample (
    id TEXT PRIMARY KEY, 
    name TEXT NOT NULL,
    content TEXT NOT NULL
);

-- 系统配置表
CREATE TABLE IF NOT EXISTS sys_config (
    key TEXT PRIMARY KEY, 
    value TEXT NOT NULL
);

-- 初始化规则示例
INSERT INTO code_sample (id,name,content) VALUES
	 ('dfebebb3-d75e-4ef7-8522-4db1cd2ba63c','mybatis-plus DO对象','### 数据库DO对象
1.对象名使用表名的驼峰格式，以DO结尾；
2.采用mybatis-plus的entity对象规范；
3.主键采用IdType.ASSIGN_ID的方式自动生成；
4.将数据库字段注释作为生成的属性的注释，使用javadoc的格式；
5.时间日期类型优先使用localdate,localdatetime来映射；
6.满足驼峰要求的字段不需要额外通过注解进行字段名映射；
~~~java
@TableName("device_attr_group")
@Data
public class DeviceAttrGroupDO {

    @TableId(value = "id", type = IdType.ASSIGN_ID)
    private String id;

    private String name;

    private String parentId;

    @TableField(fill = FieldFill.INSERT)
    private String tenantId;
}
~~~'),
	 ('6dcdd4ea-5514-4a1a-821e-5a7ced4d09a6','接口DTO对象','### API接口DTO对象：
1.对象名使用表名的驼峰格式，以DTO结尾；
2.使用springdoc格式的注解来生成字段的参数说明，数据来源于数据库表字段注释；
3.使用lombok style的注解；
4.必填项使用hibernate-validator的注解来验证；
5.时间日期类型优先使用localdate,localdatetime来映射
示例：
~~~java
@Data
public class UserDTO {

    private String id;

    @Schema(description = "用户账户")
    private String account;

    @Schema(description = "用户名称")
    private String name;

    @Schema(description = "创建时间")
    private LocalDate createTime;
}  
~~~');

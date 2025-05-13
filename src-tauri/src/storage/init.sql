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
	 ('dfebebb3-d75e-4ef7-8522-4db1cd2ba63c','数据库DO对象','### 数据库DO对象
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
~~~'),
	 ('338ec139-df26-4d40-98f8-51a9a0a27ee8','Converter对象','### 接口DTO和数据库DO对象的互相转换的Converter对象
1.  使用MapStruct框架，用于将数据库的DO对象转换为DTO对象，或者将DTO对象转换为数据库的DO对象；
2.  对象的名称为XXXConverter（例如DeviceConverter ）
3.  字段不完全匹配时，仅处理匹配的字段，忽略错误
 举例：
~~~java
import org.mapstruct.factory.Mappers;

@Mapper(unmappedTargetPolicy = ReportingPolicy.IGNORE)
public interface DeviceConverter {

    DeviceConverter INSTANCE = Mappers.getMapper(DeviceConverter.class);

    DeviceDO toDataObj(DeviceDTO dto);

    DeviceDTO toDTO(DeviceDO deviceDO);

}
~~~

'),
	 ('5c5ff59e-9d6f-4ac9-81a1-304f95698ed8','数据库mapper对象','### 数据库mapper对象
1. mybatis-plus的mapper接口，用于封装对某个DO对象的数据库操作；

~~~java
import com.baomidou.mybatisplus.core.mapper.BaseMapper;

public interface BaseDeviceCheckPointMapper extends BaseMapper<BaseDeviceCheckPointDO> {

}
~~~'),
	 ('ec69f9a9-4607-4f97-bac8-ac0908f9fd94','Service对象','### Service

1. 对象名使用表名的驼峰格式，以Service结尾，用于封装对数据库的相关业务操作，底层使用Myabtis-Plus的Mapper对象操作数据库；
2. 覆盖数据库的增，删，改，按ID查询详情，分页查询列表的相关操作；
3. create，edit接收DTO对象，并利用Converter对象转换成对应的DO对象，并调用Mapper对象进行数据库操作；
4. 如果当前上下文中存在PageQuery对象的相关信息，则生成分页查询方法。该方法接收一个分页查询的PageQuery对象作为参数，进行分页查询，并返回分页后的查询结果（使用mybatis-plus的分页插件）；
5. 所有涉及数据库的操作都需要添加事务注解；


举例：
~~~java

@Service
@RequiredArgsConstructor
public class DeviceService {

    private final DeviceMapper deviceMapper;

    @Transactional
    @Override
    public String create(DeviceDTO command) {
        DeviceDO deviceDO = DeviceConverter.INSTANCE.toDataObj(command);
        deviceMapper.insert(deviceDO);
        return deviceDO.getId();
    }

    @Transactional
    @Override
    public void delete(String id) {
        deviceMapper.deleteById(id);
    }

    @Transactional
    @Override
    public void edit(DeviceDTO command) {
        DeviceDO deviceDO = DeviceConverter.INSTANCE.toDataObj(command);
        deviceMapper.updateById(orderDO);
    }

    @Override
    public DeviceDTO detail(String id) {
        DeviceDO deviceDO = deviceMapper.selectById(id);
        if(deviceDO==null){
            return null;
        }
        return DeviceConverter.INSTANCE.toDTO(deviceDO);
    }

    @Override
    public IPage<DeviceDTO> page(DevicePageQuery query) {
        var queryWrapper = Wrappers.<DeviceDO>lambdaQuery()
                .like(StrUtil.isNotBlank(query.getOrderNo()), DeviceMaintenanceOrderDO::getOrderNo, query.getOrderNo())
                .like(StrUtil.isNotBlank(query.getCustomerName()), DeviceMaintenanceOrderDO::getCustomerName, query.getCustomerName())
                .like(StrUtil.isNotBlank(query.getDeviceName()), DeviceMaintenanceOrderDO::getDeviceName, query.getDeviceName());
        return deviceMapper.selectPage(new Page<>(query.getCurrent(), query.getSize()),
            queryWrapper).convert(DeviceMaintenanceOrderConverter.INSTANCE::toDTO);
    }

}
~~~'),
	 ('cb51e146-f673-470c-bb2b-694023cbd50a','分页接口查询对象','### 用于API 分页接口的查询对象
1. 封装了需要'),
	 ('43c93804-cd08-4fc1-b604-a9568491c32e','Controller对象','### 封装API接口的Controller对象
1.  对象名使用表名的驼峰格式，以Controller结尾；
2.  采用SpringBoot的相关技术框架，并满足RESTFUL规范的要求；
3.  如果上下文中存在Service对象的定义，则根据Service对象的相关方法生成API接口, 否则不生成任何接口；
4.  接口参数（DTO对象）应该采用Spring validation的规范进行校验
5.   接口的API地址以“/api/${实体对象名}”开头，例如“/api/device”
示例
~~~java
@RestController
@RequestMapping("/api/device")
@RequiredArgsConstructor
public class DeviceController {

    private final DeviceService deviceService;

    /**
     * 创建
     */
    @PostMapping
    public void create(@Validated @RequestBody DeviceDTO deviceDTO) {
        deviceService.create(deviceDTO);
    }

    /**
     * 更新
     */
    @PutMapping
    public void update(@Validated @RequestBody DeviceDTO deviceDTO) {
        deviceService.update(deviceDTO);
    }

    /**
     * 删除
     */
    @DeleteMapping("/{id}")
    public void delete(@PathVariable String id) {
        deviceService.delete(id);
    }

    /**
     * 分页查询
     */
    @PostMapping("/page")
    public IPage<DeviceDTO>  page(@RequestBody DevicePageQuery pageQuery) {
        return   deviceService.page(pageQuery)
    }

}
~~~');

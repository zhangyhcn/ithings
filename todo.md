common中增加一个device-core,把物模型的定义解析等增加进去，设备创建的时候根据物模型定义出自己的数据，driver是根据物模型和具体设备通信协议实现的。然后driver将解析后的数据通过zmq发送给device-core.读取一下doc目录下的model.md和driver.md，了解设备和driver的关系。设备有通用的物模型，driver是为了适配不同厂家，不同通信协议的目的。目前我们已经实现了简单的driver和device。现在在设备测需要实现一个完善的物模型，设备创建后，根据物模型和驱动，读取数据，运行规则，状态机等，根据物模型发送event和提供service服务。event通过远程publisher,发送，service通过远程subscriber提供。



每个driver应该有一个deviceresprofile管理的功能，driver启动后都要监听zmq，等待device发送配置。driver监听配置变化后，添加到自己配置里面，然后优化生成和设备（通信）通信数据，解析数据，返回给device.....





8、完善设备配置文件
1）设备配置文件里增加tenant_id,org_id,site_id,namespace_id,device_id.
2）设备文件应该支持多个设备，每个设备有物模型行信息和driver配置信息。
3) 同一类的设备信息都放到一个文件里，创建一个容器，共享一个driver,可以通过容器名查询。
4） 创建新设备时，就是配置文件中增加了一组，然后kubectl apply,容器重新加载配置。
5） device有设备管理功能。根据配置文件创建和管理设备。然后将配置发送给driver
6) driver根据device_instance_id.层级上报数据
7） device监听数据，然后通过远程publisher发送数据到mqtt或kafka，上报层级为tenant_id,org_id,site_id,namespa_id,device_instance_id.

需要改写这个功能，driver有自己的drvier管理器，每一个发送过来的driver配置都应该是不同的，主要不同点在deviceResources和deviceCommands。driver配置中应该增加一个device_instance_id,这是具体设备的id,driver根据这个来判断进行管理。接收到了重复的判断一下配置hash是否一致，没有变化，不进行处理。有变化，停止旧的，重新加载新配置并启动。


device配置为mqtt传输主题为tenant_id/org_id/site_id/device_instance_id


3.18，下一步:
1、sidecar模式
2、检查zmq接收解析
3、检查mqtt传输
4、web端配置发布
5、增加新的driver和device
6、web页面数据展示
7、增加时序数据库tdengine



3.24
节点管理
1）页面去掉创建节点按钮
2）后台通过kubectl(k8s client)获取节点信息，包括labels
3) 页面展示节点信息和labels
4) 页面可以管理节点标签


租户管理：
1）增加修改功能
2）增加镜像仓库地址配置
3）增加虚拟集群名配置

租户增加配置镜像仓库地址的功能


驱动管理，
1）增加驱动的dockerfile
2）增加python脚本创建镜像，打标签，推送镜像
3） 页面展示的时候，从指定的镜像仓库（租户配置）获取信息
4）去掉页面创建驱动的按钮。

设备实例
1）创建时可以选择设备标签


